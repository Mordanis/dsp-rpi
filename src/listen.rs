/// Module that contains code for listening to a
/// stream and redirecting the samples to a predefined
/// audio buffer

mod communication;
use cpal;
use cpal::traits::DeviceTrait;

fn create_listener(config: cpal::
