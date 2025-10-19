use overtone::transformer::{
    ExportError, NodeRef, Sink, SocketConnectionError, SocketIdx, Source, Value,
};
use std::any::Any;
use std::sync::{Arc, RwLock};
use {
    overtone::transformer::Node,
    std::path::{Path, PathBuf},
};

fn package<N: Node + 'static>(node: N) -> NodeRef {
    Arc::new(RwLock::new(node))
}
macro_rules! connect {
    ($a:expr, $a_out:expr, $b_in:expr, $b:expr) => {
        {$b.write().unwrap().connect($b_in, $a, $a_out).unwrap()}
    };
}

fn main() {
    let base = 261.63;

    let n0 = WaveGeneratorNode::new(base);
    let n0 = package(n0);
    let n1 = WaveGeneratorNode::new(base * 5.0/4.0);
    let n1 = package(n1);
    let n2 = WaveGeneratorNode::new(base * 3.0/2.0);
    let n2 = package(n2);

    let c1 = CombineNode::new();
    let mut c1 = package(c1);
    let c2 = CombineNode::new();
    let mut c2 = package(c2);

    let g1 = GainNode::new(0.25);
    let mut g1 = package(g1);

    let nz = WAVExporter::new("./examples/nodes/tune.wav");
    let mut nz = package(nz);

    {
        connect!(n0, 0, 0, c1.clone());
        connect!(n1, 0, 1, c1.clone());
        connect!(c1, 0, 0, c2.clone());
        connect!(n2, 0, 1, c2.clone());
        connect!(c2, 0, 0, g1.clone());
        connect!(g1, 0, 0, nz.clone());
    }

    {
        nz.write().unwrap().as_sink().unwrap().drain();
    }
}

struct WaveGeneratorNode {
    frequency: f32,
}
impl WaveGeneratorNode {
    pub fn new(frequency: f32) -> Self {
        WaveGeneratorNode { frequency }
    }
}
impl Node for WaveGeneratorNode {
    fn connect(
        &mut self,
        to_socket: SocketIdx,
        from_node: NodeRef,
        from_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        Err(SocketConnectionError::NoSuchSocket)
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        // There are no sockets, but there's no reason to emit an error or anything.
    }

    fn as_source(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        if from_socket != 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }
        let audio_source = WaveGenerator {
            frequency: self.frequency,
        };
        let audio_source: Box<dyn Source<Item = AudioPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}

pub struct WaveGenerator {
    frequency: f32,
}
impl Source for WaveGenerator {
    type Item = AudioPcm;

    fn pull(&mut self) -> Self::Item {
        AudioPcm::example(self.frequency)
    }
}

/// A struct containing PCM audio.
/// Ideally, this would be a `RealTimeStream` so that it can be
/// previewed or exported in real time and concurrently.
pub struct AudioPcm {
    pub sample_rate: usize,
    pub content: Vec<f32>,
}
impl Value for AudioPcm {
    fn get_format_name(&self) -> String {
        "audio/pcm".to_string()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl AudioPcm {
    pub fn example(frequency: f32) -> Self {
        let sample_rate = 41000;
        let mut content = Vec::with_capacity(sample_rate);

        for i in 0..sample_rate {
            let t = i as f32 / sample_rate as f32;
            let frequency = frequency * std::f32::consts::TAU;
            let amplitude = (1.0f32 - 0.985f32).powf(t);
            let sample = (t * frequency).sin();
            let sample = sample.signum();
            let sample = sample * amplitude;
            content.push(sample);
        }

        Self {
            sample_rate,
            content,
        }
    }
}

struct GainNode {
    gain: f32,
    source: Option<Box<dyn Source<Item = AudioPcm>>>,
}
impl GainNode {
    pub fn new(gain: f32) -> Self {
        GainNode { gain, source: None }
    }
}

impl Node for GainNode {
    fn connect(
        &mut self,
        to_socket: SocketIdx,
        from_node: NodeRef,
        from_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        if !(to_socket == 0 || to_socket == 1) {
            return Err(SocketConnectionError::NoSuchSocket);
        }

        let mut from_node = from_node.write().unwrap();
        self.source = Some(from_node.try_get_source(from_socket).unwrap());
        Ok(())
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        self.source = None;
    }

    fn as_source(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        if !from_socket == 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }

        struct Gain {
            gain: f32,
            source: Box<dyn Source<Item = AudioPcm>>,
        }
        impl Source for Gain {
            type Item = AudioPcm;

            fn pull(&mut self) -> Self::Item {
                let mut frame = self.source.pull();
                for sample in frame.content.iter_mut() {
                    *sample *= self.gain;
                }
                frame
            }
        }

        let audio_source = Gain {
            gain: self.gain,
            source: self.source.take().unwrap(),
        };
        let audio_source: Box<dyn Source<Item = AudioPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}

struct CombineNode {
    source1: Option<Box<dyn Source<Item = AudioPcm>>>,
    source2: Option<Box<dyn Source<Item = AudioPcm>>>,
}
impl CombineNode {
    pub fn new() -> Self {
        Self {
            source1: None,
            source2: None,
        }
    }
}
impl Node for CombineNode {
    fn connect(&mut self, to_socket: SocketIdx, from_node: NodeRef, from_socket: SocketIdx) -> Result<(), SocketConnectionError> {
        let mut from_node = from_node.write().unwrap();
        match to_socket {
            0 => {
                self.source1 = Some(from_node.try_get_source(from_socket).unwrap());
                Ok(())
            }
            1 => {
                self.source2 = Some(from_node.try_get_source(from_socket).unwrap());
                Ok(())
            }
            _ => Err(SocketConnectionError::NoSuchSocket),
        }
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        match socket {
            0 => {
                self.source1 = None
            }
            1 => {
                self.source2 = None
            }
            _ => (),
        }
    }

    fn as_source(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        if !from_socket == 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }

        struct Combine {
            source1: Box<dyn Source<Item = AudioPcm>>,
            source2:  Box<dyn Source<Item = AudioPcm>>,
        }
        impl Source for Combine {
            type Item = AudioPcm;

            fn pull(&mut self) -> Self::Item {
                let frame1 = self.source1.pull();
                let frame2 = self.source2.pull();
                AudioPcm {
                    sample_rate: frame1.sample_rate,
                    content: frame1.content.iter().zip(frame2.content.iter()).map(|(a, b)| *a + *b).collect(),
                }
            }
        }

        let audio_source = Combine {
            source1: self.source1.take().unwrap(),
            source2: self.source2.take().unwrap(),
        };
        let audio_source: Box<dyn Source<Item = AudioPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}

struct WAVExporter {
    file: PathBuf,
    sample_rate: usize,
    source: Option<Box<dyn Source<Item = AudioPcm>>>,
}
impl WAVExporter {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        WAVExporter {
            file: PathBuf::from(path.as_ref()),
            sample_rate: 41000,
            source: None,
        }
    }
}
impl Node for WAVExporter {
    fn connect(
        &mut self,
        to_socket: SocketIdx,
        from_node: NodeRef,
        from_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        if to_socket != 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }

        self.source = Some(from_node.write().unwrap().try_get_source(from_socket)?);
        Ok(())
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        self.source = None;
    }

    fn as_source(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        Err(SocketConnectionError::NoSuchSocket)
    }

    fn as_sink(&mut self) -> Option<&mut dyn Sink> {
        Some(self)
    }
}
impl Sink for WAVExporter {
    fn drain(&mut self) -> Result<(), ExportError> {
        let location = &self.file;
        let audio_pcm = self.source.as_mut().unwrap().pull();
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: audio_pcm.sample_rate as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(location, spec).expect("Failed to write.");

        for sample in audio_pcm.content.iter().copied() {
            let sample16: i16 = (sample * (i16::MAX as f32)) as i16;
            writer.write_sample(sample16).unwrap();
        }

        writer
            .finalize()
            .expect("Error finalising to write the wav file.");

        Ok(())
    }
}
