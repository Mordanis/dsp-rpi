extern crate num;
extern crate rustfft;
extern crate hound;

use std::fs::File;
use std::io::prelude::*;

mod read_kernel;
mod conv;

fn main() {
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
