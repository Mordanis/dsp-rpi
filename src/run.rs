/// Module that runs DSP on an iterator
use conv;

pub struct DSP {
    convolver: conv::Convolver,
    in_buffer: Vec<f32>
}

impl DSP {
    pub fn new(kernel: Vec<f32>) {
        let buff: Vec<f32> = Vec::with_capacity(kernel.len());
        let buffiter = buff.into_iter();
        DSP {
            convolver: conv::Convolver::new(kernel),
            in_buffer: Vec::with_capacity(kernel.len())
        }
    }

    pub fn do_dsp(inp_stream: Iterator<f32>) -> Iterator<f32> {
        let mut in_buff: Vec<f32> = 
    }
}
