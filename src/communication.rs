use std::sync::mpsc;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use cpal;


pub struct AudioBuffer {
    data: Arc<Mutex<Vec<f32>>>,
    stream: Option<cpal::Stream>
}

impl AudioBuffer {
    pub fn new() -> Self {
        AudioBuffer { 
            data: Arc::new(Mutex::new(Vec::new())),
            stream: None
        }
    }

    pub fn feed_data(&mut self, sink: mpsc::Receiver<Vec<f32>>) {
        let feeder_data = Arc::clone(&mut self.data);
        thread::spawn(
            move || {
                let sr = super::constants::SAMPLE_RATE;
                let duration = time::Duration::from_secs_f32(sr);
                loop {
                    thread::sleep(duration);
                    {
                        let res = sink.recv();
                        match res {
                            Ok(mut data) => {
                                let mut local_data = feeder_data.lock().unwrap();
                                local_data.append(&mut data);
                            },
                            Err(_) => {}
                        };
                    }
                }
            }
        );
    }

    pub fn trim_data(&mut self) {
        let trim_data = Arc::clone(&mut self.data);
        thread::spawn(
            move || {
                let duration = super::constants::SYNC_DURATION;
                let mbs = super::constants::MAX_BUFFER_SIZE;
                loop {
                    thread::sleep(duration);
                    {
                        let mut local_data = trim_data.lock().unwrap();
                        let num_elems = local_data.len();
                        if num_elems > mbs {
                            let start_idx = num_elems - mbs;
                            local_data.drain(..start_idx);
                       }
                    }
                }
            }
        );
    }

    pub fn get_data(&self) -> Vec<f32> {
        let data_local = Arc::clone(&self.data);
        let data = data_local.lock().unwrap();
        data.clone()
    }

    /// Exists so we can create a buffer in a function and return
    /// it without the stream going out of scope and being removed
    pub fn give_stream(&mut self, stream: cpal::Stream) {
        self.stream = Some(stream);
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_comm() {
        let mut vec_container = super::AudioBuffer::new();
        let sink = externally_create_data();
        vec_container.feed_data(sink);
        vec_container.trim_data();
        let duration = std::time::Duration::from_secs(2);
        std::thread::sleep(duration);
        println!("Hello, world!");
        println!("data size is: {}", vec_container.get_data().len());
        println!("data is: {:?}", vec_container.get_data());
    }

    fn externally_create_data() -> std::sync::mpsc::Receiver<Vec<f32>> {
        let (source, sink) = std::sync::mpsc::channel();
        std::thread::spawn(
            move || {
                let mut out = Vec::new();
                for i in 0..128 {
                    let val = (i % 2) * 36 * (i % 36);
                    out.push(val as f32);
                }
                let duration = std::time::Duration::from_secs_f32(5.0 / 44100.0);
                loop {
                    let _res = source.send(out.clone());
                    std::thread::sleep(duration);
                }
            }
        );
        sink
    }
}
