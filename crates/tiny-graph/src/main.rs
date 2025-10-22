use {
    cpal::{
        FromSample,
        traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _},
    },
    dasp::Sample,
    directed::*,
    std::{
        sync::{Arc, Mutex},
        time::Instant,
    },
};

static SAMPLE_RATE: usize = 41000;

#[derive(Clone, Copy, PartialEq)]
struct PlayBackState {
    pub tick: u32,
    pub sample_rate: u32,
    pub time: f64,
}
impl PlayBackState {
    pub fn advance_tick(&mut self) {
        self.tick += 1;
        self.time = (self.tick as f64) / (self.sample_rate as f64);
    }
}

#[stage(state(inner: PlayBackState))]
fn PlayBackNode() -> PlayBackState {
    inner.advance_tick();
    *inner
}

#[stage]
fn SineGen(playback_state: PlayBackState, frequency: f64) -> f64 {
    (playback_state.time * frequency * std::f64::consts::TAU).sin()
}

#[stage]
fn Gain(input: f64, amount: f64) -> f64 {
    input * amount
}

fn main() {
    let registry = Arc::new(Mutex::new(Registry::new()));

    let mut registry_lock = registry.lock().expect("Couldn't lock my poor boy.");

    let playbacknode = registry_lock.register_with_state(
        PlayBackNode,
        PlayBackNodeState {
            inner: PlayBackState {
                tick: 0,
                sample_rate: SAMPLE_RATE as u32,
                time: 0.0,
            },
        },
    );
    let sine_value = registry_lock.value(440.0);
    let sine = registry_lock.register(SineGen);
    let gain_value = registry_lock.value(2.0);
    let gain = registry_lock.register(Gain);

    let graph = Arc::new(
        graph! {
            nodes: (playbacknode, sine_value, sine, gain_value, gain),
            connections: {
                playbacknode => { sine: playback_state }
                sine_value => { sine: frequency }
                sine =>{ gain: input }
                gain_value => { gain: amount }
            }
        }
        .unwrap(),
    );
    drop(registry_lock);

    let start = Instant::now();
    playback(registry, gain, graph).unwrap();
    let end = Instant::now();

    println!("Took {:?}", end - start);
}

fn export(registry: &mut Registry, gain: NodeId<Gain>, graph: Graph) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    for _ in 0..(SAMPLE_RATE * 10) {
        let result = graph.execute(registry, gain).unwrap();
        writer
            .write_sample(result.0.unwrap_or(0.0).to_sample::<i16>())
            .unwrap();
    }
    writer.finalize().unwrap();
}

/// Minimal real-time playback of a directed graph using cpal.
fn playback(
    registry: Arc<Mutex<Registry>>,
    sink: NodeId<Gain>,
    graph: Arc<Graph>,
) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config()?;

    println!("Using device: {}", device.name()?);
    println!("Sample format: {:?}", config.sample_format());
    println!("Sample rate: {}", config.sample_rate().0);

    let sample_rate = config.sample_rate().0 as usize;

    let err_fn = |err| eprintln!("Stream error: {}", err);

    fn build_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        registry: Arc<Mutex<Registry>>,
        graph: Arc<Graph>,
        sink: NodeId<Gain>,
        _sample_rate: usize,
        err_fn: impl Fn(cpal::StreamError) + Send + 'static,
    ) -> Result<cpal::Stream, anyhow::Error>
    where
        T: cpal::Sample + FromSample<f64> + cpal::SizedSample,
    {
        let channels = config.channels as usize;

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut registry = registry.lock().unwrap();

                for frame in data.chunks_mut(channels) {
                    let result = graph.execute(&mut registry, sink).unwrap();
                    let sample = result.0.unwrap_or(0.0);
                    let value: T = sample.to_sample();

                    for ch in frame {
                        *ch = value;
                    }
                }
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(
            &device,
            &config.into(),
            registry,
            graph,
            sink,
            sample_rate,
            err_fn,
        )?,
        cpal::SampleFormat::I16 => build_stream::<i16>(
            &device,
            &config.into(),
            registry,
            graph,
            sink,
            sample_rate,
            err_fn,
        )?,
        cpal::SampleFormat::U16 => build_stream::<u16>(
            &device,
            &config.into(),
            registry,
            graph,
            sink,
            sample_rate,
            err_fn,
        )?,
        _ => unimplemented!(),
    };

    stream.play()?;
    println!("Playing for 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Done.");
    Ok(())
}
