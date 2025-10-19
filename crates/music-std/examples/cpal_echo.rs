use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cpal::StreamConfig;

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    // Recording your beautiful voice.
    let input_device = host.default_input_device().expect("No input device");
    let input_config: StreamConfig = input_device.default_input_config()?.into();

    let sample_rate = input_config.sample_rate.0 as usize;
    let channels = input_config.channels as usize;

    let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let buffer_clone = buffer.clone();

    let input_stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _: &_| {
            let mut buf = buffer_clone.lock().unwrap();
            buf.extend_from_slice(data);
        },
        move |err| eprintln!("Input stream error: {:?}", err),
        None
    )?;

    input_stream.play()?;
    println!("Recording 1 second...");
    thread::sleep(Duration::from_secs(1));
    drop(input_stream); // stop recording

    let recorded = buffer.lock().unwrap().clone();
    println!("Recording done, {} samples captured", recorded.len());

    // --- Playing it back !! ---
    let output_device = host.default_output_device().expect("No output device");
    let output_config = output_device.default_output_config()?.into();

    let output_stream = output_device.build_output_stream(
        &output_config,
        move |out: &mut [f32], _: &_| {
            for (i, sample) in out.iter_mut().enumerate() {
                *sample = recorded.get(i).copied().unwrap_or(0.0);
            }
        },
        move |err| eprintln!("Output stream error: {:?}", err),
        None
    )?;

    println!("Playing back...");
    output_stream.play()?;
    thread::sleep(Duration::from_secs(1));

    Ok(())
}
