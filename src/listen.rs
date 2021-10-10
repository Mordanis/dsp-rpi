/// Module that contains code for listening to a
/// stream and redirecting the samples to a predefined
/// audio buffer

use cpal::traits::{HostTrait, DeviceTrait};
use cpal;
use super::communication;
use std::sync::mpsc;
use std::io::{stdin, stdout, Write};

/// Create a listener on the device with the given configuration.
/// Returns an audio buffer that will be used by the processing
/// thread.
pub fn create_listener(
    device: cpal::Device,
    config: cpal::StreamConfig
) -> communication::AudioBuffer {

    let mut buffer = communication::AudioBuffer::new();

    let sender: mpsc::Sender<Vec<f32>>;
    let receiver: mpsc::Receiver<Vec<f32>>;
    let channel = mpsc::channel();
    sender = channel.0;
    receiver = channel.1;

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let data_vec = Vec::from(data);
            sender.send(data_vec).unwrap();
        },
        move |err| {
            println!("got error {}", err);
            // handle errors
        }
    ).unwrap();
    
    buffer.feed_data(receiver);
    buffer.give_stream(stream);
    buffer
}


pub fn create_device(host: cpal::Host) -> cpal::Device {
    let devices = host.devices().unwrap();
    let device_names = devices
        .map(|dev| dev.name().unwrap())
        .collect::<Vec<String>>();
    let dev = match device_names.len() {
        1 => host.devices().unwrap().next().unwrap(),
        0 => panic!("No available devices on the host!"),
        _ => {
            let idx = get_user_device_choice(&host);
            host.devices().unwrap().nth(idx).unwrap()
        }
    };
    dev
}

pub fn create_host() -> cpal::Host {
    let available_hosts = cpal::available_hosts();
    let host_id = match available_hosts.len() {
        1 => available_hosts[0],
        0 => panic!("No hosts found"),
        _ => {
            let user_idx = get_user_host_choice(&available_hosts);
            available_hosts[user_idx]
        }
    };

    cpal::host_from_id(host_id).unwrap()
}


pub fn create_listener_config(device: &cpal::Device) -> cpal::SupportedStreamConfig {
    let configs = device.supported_input_configs().unwrap();
    let configs: Vec<cpal::SupportedStreamConfigRange> = configs.into_iter().collect();
    match configs.len() {
        1 => configs[0].clone().with_sample_rate(cpal::SampleRate(44_100)),
        0 => panic!("Unable to create listening configuration for device {:?}", device.name().unwrap()),
        _ => {
            let idx = get_user_config_choice(&device);
            configs[idx].clone().with_sample_rate(cpal::SampleRate(44_100))
        }
    }
}


pub fn create_output_config(device: &cpal::Device) -> cpal::SupportedStreamConfig {
    let configs = device.supported_output_configs().unwrap();
    let configs: Vec<cpal::SupportedStreamConfigRange> = configs.into_iter().collect();
    match configs.len() {
        1 => configs[0].clone().with_sample_rate(cpal::SampleRate(44_100)),
        0 => panic!("Unable to create listening configuration for device {:?}", device.name().unwrap()),
        _ => {
            let idx = get_user_config_choice(&device);
            configs[idx].clone().with_sample_rate(cpal::SampleRate(44_100))
        }
    }
}

fn get_user_host_choice(hosts: &Vec<cpal::HostId>) -> usize {
    loop {
        println!("please choose a host for DSP to run on");
        let mut i = 0;
        for host in hosts.clone() {
            println!("Host {}: {}", i, host.name());
            i += 1;
        }
        let mut user_inp = String::new();

        // read in the input string and remove the newline character
        stdin().read_line(&mut user_inp).unwrap();
        user_inp.remove(user_inp.len() - 1);

        match user_inp.parse::<usize>() {
            Ok(i) => {
                if i < hosts.len() {
                    return i;
                }
                else {
                    println!("please choose one of the available hosts.");
                }
            },
            Err(err) => println!("Unable to parse due to {:?}", err)
        };
    };
}


fn get_user_device_choice(host: &cpal::Host) -> usize {
    loop {
        println!("please choose a device for DSP to run on");
        let mut i_max = 0;
        for (i, dev) in host.devices().unwrap().enumerate() {
            println!("Device {}: {}", i, dev.name().unwrap());
            i_max += 1;
        }
        let mut user_inp = String::new();

        // read in the input string and remove the newline character
        stdin().read_line(&mut user_inp).unwrap();
        user_inp.remove(user_inp.len() - 1);

        match user_inp.parse::<usize>() {
            Ok(i) => {
                if i <= i_max {
                    return i;
                }
                else {
                    println!("please choose one of the available devices.");
                }
            },
            Err(err) => println!("Unable to parse due to {:?}", err)
        };
    };
}


fn get_user_config_choice(device: &cpal::Device) -> usize {
    loop {
        println!("please choose a configuration for the DSP input stream");
        let mut i_max = 0;
        for (i, cfg) in device.supported_input_configs().unwrap().enumerate() {
            println!("Configuration {}: {:?}", i, cfg);
            i_max += 1;
        }
        let mut user_inp = String::new();

        // read in the input string and remove the newline character
        stdin().read_line(&mut user_inp).unwrap();
        user_inp.remove(user_inp.len() - 1);

        match user_inp.parse::<usize>() {
            Ok(i) => {
                if i <= i_max {
                    return i;
                }
                else {
                    println!("please choose one of the available configurations.");
                }
            },
            Err(err) => println!("Unable to parse due to {:?}", err)
        };
    };
}


#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use cpal::traits::{DeviceTrait, StreamTrait};
    use std::time;

    #[test]
    fn create_devices() {
        let host = super::create_host();
        let device = super::create_device(host);
        let listen_config = super::create_listener_config(&device);
        let output_config = super::create_output_config(&device);
        let (sender, receiver) = mpsc::channel();
        println!("building input stream");
        let input_stream = device.build_input_stream(
            &listen_config.into(),
            move |data: &[f32], _cb: &cpal::InputCallbackInfo| {
                sender.send(data.to_vec()).unwrap();
            },
            |err| eprintln!("got error {}", err)
        ).unwrap();
        let output_stream = device.build_output_stream(
            &output_config.into(),
            move |data: &mut [f32], _cb: &cpal::OutputCallbackInfo| {
                let received = match receiver.recv() {
                    Ok(samples) => samples,
                    Err(_) => Vec::new()
                };
                let mut recv_iter = received.iter();
                for output_sample in data.iter_mut() {
                    let next = match recv_iter.next() {
                        Some(s) => s,
                        None => break
                    };
                    *output_sample = *next;
                }
            },
            |err| eprintln!("got error in writing to stream\n{}", err)
            ).unwrap();

        input_stream.play().unwrap();
        output_stream.play().unwrap();
        
        std::thread::sleep(time::Duration::from_millis(100));


        assert!(2 + 2 == 4);
    }
}
