extern crate num;
extern crate rustfft;
extern crate hound;
use std::sync::Arc;
use num::Complex;
use rustfft::{FftPlanner, num_complex, Fft};

use std::fs::File;
use std::io::prelude::*;

mod read_kernel;

struct Convolver {
    kernel_fourier: Vec<num_complex::Complex64>,
    fft: Arc<dyn Fft<f64>>,
    ift: Arc<dyn Fft<f64>>,
    scratch:Vec<Complex<f64>>
}

impl Convolver {
    pub fn new(kernel: Vec<f32>) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(kernel.len());
        let ift = planner.plan_fft_inverse(kernel.len());

        let norm_factor = 1f32 / (kernel.len() as f32).sqrt(); 
        let mut kernel_complex: Vec<num_complex::Complex64> = kernel
            .iter()
            .map(|x| num_complex::Complex64::new((*x as f64) * norm_factor as f64, 0.0))
            .collect::<Vec<num_complex::Complex64>>();


        fft.process(&mut kernel_complex);

        let scratch: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); kernel.len()];

        Convolver {
            kernel_fourier: kernel_complex,
            fft,
            ift,
            scratch
        }
    }

    pub fn conv_with_kernel(&mut self, sample: Vec<f32>) -> Vec<f32> {
        let mut sample_complex: Vec<num_complex::Complex64> = sample
            .into_iter()
            .map(|x| num_complex::Complex64::new(x as f64, 0.0))
            .collect::<Vec<num_complex::Complex64>>();

        self.fft.process_with_scratch(&mut sample_complex[..], &mut self.scratch[..]);

        let mut conved: Vec<num_complex::Complex64> = Vec::with_capacity(sample_complex.len());
        for i in 0..sample_complex.len() {
            let val = sample_complex[i] * self.kernel_fourier[i];
            conved.push(val);
        }

        println!("first 10 of conved are {:?}", conved[..10].to_vec());
        println!("first 10 of inp are {:?}", sample_complex[..10].to_vec());

        let norm_factor = 1.0 / (conved.len() as f32);
        self.ift.process(&mut conved);
        conved
            .into_iter()
            .map(|x| x.re as f32 * norm_factor as f32)
            .collect::<Vec<f32>>()

   }
}


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

    let mut convolver = Convolver::new(kernel);

    let mut file = File::create("original.dat").unwrap();
    let contents = format!("{:?}", first_sample);
    let _res = write!(file, "{}", contents);
    let start = std::time::Instant::now();
    let mut out;
    for i in 0..20 {
        out = convolver.conv_with_kernel(first_sample.clone());
        if i == 0 {
            let mut file = File::create("conved.dat").unwrap();
            let contents = format!("{:?}", out);
            let _res2 = write!(file, "{}", contents);
        }
    }
    let elapsed = start.elapsed();
    println!("{:?} us elapsed", elapsed.as_micros());
    println!("Hello, world!");
}
