use std::sync::mpsc;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};

struct VecContainer {
    data: Arc<Mutex<Vec<f32>>>
}

impl VecContainer {
    fn new() -> Self {
        VecContainer { 
            data: Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn feed_data(&mut self, sink: mpsc::Receiver<Vec<f32>>) {
        let feeder_data = Arc::clone(&mut self.data);
        thread::spawn(
            move || {
                let duration = time::Duration::from_secs_f32(1.0 / 44100.0);
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

    fn trim_data(&mut self) {
        let trim_data = Arc::clone(&mut self.data);
        thread::spawn(
            move || {
                let duration = time::Duration::from_millis(10);
                loop {
                    thread::sleep(duration);
                    {
                        let mut local_data = trim_data.lock().unwrap();
                        let num_elems = local_data.len();
                        if num_elems > 500 {
                            let start_idx = num_elems - 500;
                            local_data.drain(..start_idx);
                       }
                    }
                }
            }
        );
    }

    fn get_data(&self) -> Vec<f32> {
        let data_local = Arc::clone(&self.data);
        let data = data_local.lock().unwrap();
        data.clone()
    }
}

fn test_comm() {
    let mut vec_container = VecContainer::new();
    let sink = externally_create_data();
    vec_container.feed_data(sink);
    vec_container.trim_data();
    let duration = time::Duration::from_secs(20);
    thread::sleep(duration);
    println!("Hello, world!");
    println!("data size is: {}", vec_container.get_data().len());
    println!("data is: {:?}", vec_container.get_data());
}

fn externally_create_data() -> mpsc::Receiver<Vec<f32>> {
    let (source, sink) = mpsc::channel();
    thread::spawn(
        move || {
            let mut out = Vec::new();
            out.push(0.0);
            out.push(1.0);
            out.push(2.0);
            out.push(3.0);
            out.push(4.0);
            let duration = time::Duration::from_secs_f32(5.0 / 44100.0);
            loop {
                let _res = source.send(out.clone());
                thread::sleep(duration);
            }
        }
    );
    sink
}
