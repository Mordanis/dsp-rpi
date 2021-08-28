use std::sync::Arc;
use num::Complex;
use rustfft::{FftPlanner, num_complex, Fft};


pub struct Convolver {
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
        let kernel_complex: Vec<num_complex::Complex64> = kernel
            .iter()
            .map(|x| num_complex::Complex64::new((*x as f64) * norm_factor as f64, 0.0))
            .collect::<Vec<num_complex::Complex64>>();


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

        let norm_factor = 1.0 / (conved.len() as f32);
        self.ift.process_with_scratch(&mut conved, &mut self.scratch[..]);
        conved
            .into_iter()
            .map(|x| x.re as f32 * norm_factor as f32)
            .collect::<Vec<f32>>()

   }
}



