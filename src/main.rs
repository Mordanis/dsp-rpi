extern crate num;
extern crate rustfft;
extern crate hound;

use std::fs::File;
use std::io::prelude::*;

mod read_kernel;
mod conv;
mod constants;
mod communication;
mod listen;

use std::sync::mpsc;
use cpal::traits::{DeviceTrait, StreamTrait};

fn main() {

    let host = listen::create_host();
    let device = listen::create_device(host);
    let listen_config = listen::create_listener_config(&device);
    let output_config = listen::create_output_config(&device);
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


    let mut reader = hound::WavReader::open("./apology.wav").unwrap();
    let data = reader.samples::<i32>();
    let data = data
        .into_iter()
        .map(|x| match x {
            Ok(val) => val as f32,
            Err(_) => 50000 as f32
        })
        .collect::<Vec<f32>>();

    let data = data
        .into_iter()
        .step_by(2)
        .collect::<Vec<f32>>();

    let mut max = 0.0;
    for x in data.clone() {
        if x > max {
            max = x;
        }
    }

    let data = data
        .iter()
        .map(|x| x / (max * 1.01));

    let samples = data
        .into_iter()
        .map(|x| x)
        .collect::<Vec<f32>>();

    let kernel = read_kernel::read_corrector();
    let mut file = File::create("read_kernel.dat").unwrap();
    let _res = write!(file, "{:?}", &kernel);
    let first_sample = samples[2205000..2205000 + &kernel.len()].to_vec();

    let mut convolver = conv::Convolver::new(kernel);

    let mut file = File::create("original.dat").unwrap();
    let contents = format!("{:?}", first_sample);
    let _res = write!(file, "{}", contents);
    let start = std::time::Instant::now();
    let mut out;
    for i in 0..40 {
        out = convolver.conv_with_kernel(first_sample.clone());
        if i == 0 {
            let mut file = File::create("conved.dat").unwrap();
            let contents = format!("{:?}", out);
            let _res2 = write!(file, "{}", contents);
        }
    }
    let elapsed = start.elapsed();
    println!("{:?} us elapsed", elapsed.as_micros());
    println!("{:?} us per 0.5s sample", elapsed.as_micros() as f32 / 40.0);
    println!("Hello, world!");
}
