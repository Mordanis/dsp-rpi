/// Constants file for DSP
use std::time;

// The window size for the convolutions
#[allow(dead_code)]
pub const WINDOW_SIZE: usize = 44100 / 2;

// The maximum amount of samples to hold in memory
#[allow(dead_code)]
pub const MAX_BUFFER_SIZE: usize = WINDOW_SIZE * 8;

// The sample rate for the project to use
#[allow(dead_code)]
pub const SAMPLE_RATE: f32 = 1.0 / 44100.0;

// How often the threads should communicate with each other to
// share data
#[allow(dead_code)]
pub const SYNC_DURATION: time::Duration = time::Duration::from_millis(20);
