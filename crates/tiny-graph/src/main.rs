use {
    cpal::{
        FromSample,
        traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _},
    },
    dasp::Sample,
    directed::*,
    std::{
        f64::consts::TAU,
        sync::{Arc, Mutex},
    },
};

static SAMPLE_RATE: usize = 44100;
static BLOCK_SIZE: usize = 64;
type Block<T> = [T; BLOCK_SIZE];

#[derive(Clone, Copy, PartialEq)]
struct PlayBackState {
    pub tick: u64,
    pub sample_rate: u32,
    pub time: f64,
}
impl PlayBackState {
    pub fn advance_block(&mut self) {
        self.tick += BLOCK_SIZE as u64;
        self.time = (self.tick as f64) / (self.sample_rate as f64);
    }
}

#[stage(state(inner: PlayBackState))]
fn PlayBackNode() -> PlayBackState {
    inner.advance_block();
    *inner
}

#[stage(cache_last)]
fn SineGen(playback_state: PlayBackState, frequency: f64) -> Block<f64> {
    let mut buf = [0.0; BLOCK_SIZE];
    for (idx, sample) in buf.iter_mut().enumerate() {
        let t = (playback_state.tick as f64 + idx as f64) / (playback_state.sample_rate as f64);
        *sample = (t * frequency * TAU).sin();
    }
    buf
}

#[stage]
fn Gain(input: [f64; BLOCK_SIZE], amount: f64) -> [f64; BLOCK_SIZE] {
    let mut buf = [0.0; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        buf[i] = input[i] * amount as f64;
    }
    buf
}

fn main() {
    let registry = Arc::new(Mutex::new(Registry::new()));
    let mut reg = registry.lock().unwrap();

    let playbacknode = reg.register_with_state(
        PlayBackNode,
        PlayBackNodeState {
            inner: PlayBackState {
                tick: 0,
                sample_rate: SAMPLE_RATE as u32,
                time: 0.0,
            },
        },
    );

    let sine_value = reg.value(440.0);
    let sine = reg.register(SineGen);
    let gain_value = reg.value(0.1);
    let gain = reg.register(Gain);

    let graph = Arc::new(
        graph! {
            nodes: (playbacknode, sine_value, sine, gain_value, gain),
            connections: {
                playbacknode => { sine: playback_state }
                sine_value => { sine: frequency }
                sine => { gain: input }
                gain_value => { gain: amount }
            }
        }
        .unwrap(),
    );
    drop(reg);

    println!("Starting playback...");
    playback(registry, gain, graph).unwrap();
}

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
    let err_fn = |err| eprintln!("Stream error: {}", err);

    let sample_rate = config.sample_rate().0 as usize;
    let channels = config.channels() as usize;

    fn build_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        registry: Arc<Mutex<Registry>>,
        graph: Arc<Graph>,
        sink: NodeId<Gain>,
        channels: usize,
        err_fn: impl Fn(cpal::StreamError) + Send + 'static,
    ) -> Result<cpal::Stream, anyhow::Error>
    where
        T: cpal::Sample + FromSample<f64> + cpal::SizedSample,
    {
        let mut block = [0.0; BLOCK_SIZE];
        let mut index = BLOCK_SIZE; // start beyond buffer to trigger first gen

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut registry = registry.lock().unwrap();

                for frame in data.chunks_mut(channels) {
                    if index >= BLOCK_SIZE {
                        // generate a new block
                        let result = graph.execute(&mut registry, sink).unwrap();
                        block = result.0.unwrap_or([0.0; BLOCK_SIZE]);
                        index = 0;
                    }

                    let value: T = block[index].to_sample();
                    index += 1;

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
            channels,
            err_fn,
        )?,
        cpal::SampleFormat::I16 => build_stream::<i16>(
            &device,
            &config.into(),
            registry,
            graph,
            sink,
            channels,
            err_fn,
        )?,
        cpal::SampleFormat::U16 => build_stream::<u16>(
            &device,
            &config.into(),
            registry,
            graph,
            sink,
            channels,
            err_fn,
        )?,
        _ => unimplemented!(),
    };

    stream.play()?;
    println!("Playing for 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
