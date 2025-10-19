use overtone::transformer::{ExportError, NodeRef, Sink, SocketConnectionError, SocketIdx, SocketRef, Source, Value};
use std::any::Any;
use std::sync::{Arc, RwLock};
use {
    overtone::transformer::Node,
    std::path::{Path, PathBuf},
};

use audio::AudioPcm;
mod audio;

fn new_node<N: Node + 'static>(node: N) -> NodeRef {
    Arc::new(RwLock::new(node))
}

macro_rules! connect {
    ($a:expr, $a_out:expr, $b_in:expr, $b:expr) => {{
        $b.write().unwrap().connect($b_in, $a, $a_out).unwrap()
    }};
}

fn main() {
    let base = 261.63;

    let n0 = new_node(WaveGenerator::new(base));
    let n1 = new_node(WaveGenerator::new(base * 5.0 / 4.0));
    let n2 = new_node(WaveGenerator::new(base * 3.0 / 2.0));

    let c1 = new_node(CombineNode::new());
    let c2 = new_node(CombineNode::new());

    let g1 = new_node(GainNode::new(0.25));

    let nz = new_node(WAVExporter::new("./examples/nodes/tune.wav"));

    {
        connect!(n0, 0, 0, c1.clone());
        connect!(n1, 0, 1, c1.clone());
        connect!(c1, 0, 0, c2.clone());
        connect!(n2, 0, 1, c2.clone());
        connect!(c2, 0, 0, g1.clone());
        connect!(g1, 0, 0, nz.clone());
    }

    {
        nz.write()
            .unwrap()
            .as_sink()
            .expect("Node could not be converted to a Sink.")
            .drain()
            .expect("Couldn't drain the sink.");
    }
}

// -- WAVE GENERATOR --

struct WaveGenerator {
    frequency: f32,
}
impl WaveGenerator {
    pub fn new(frequency: f32) -> Self {
        WaveGenerator { frequency }
    }
}
impl Node for WaveGenerator {
    fn connect(
        &mut self,
        to_socket: SocketIdx,
        from_node: NodeRef,
        from_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        Err(SocketConnectionError::NoSuchSocket)
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        // There are no input sockets to disconnect, but there's no reason to emit an error or anything.
    }

    fn as_source(&mut self, from_out_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        if from_out_socket != 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }
        pub struct InnerSource {
            frequency: f32,
        }
        impl Source for InnerSource {
            type Item = AudioPcm;

            fn pull(&mut self) -> Self::Item {
                AudioPcm::example(self.frequency)
            }
        }
        let audio_source = InnerSource {
            frequency: self.frequency,
        };
        let audio_source: Box<dyn Source<Item = AudioPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}

struct GainNode {
    gain: f32,
    source: Option<SocketRef>,
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
        self.source = Some(SocketRef(from_node, from_socket));
        Ok(())
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        self.source = None;
    }

    fn as_source(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        if from_socket != 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }

        struct InnerSource {
            gain: f32,
            source: Box<dyn Source<Item = AudioPcm>>,
        }
        impl Source for InnerSource {
            type Item = AudioPcm;

            fn pull(&mut self) -> Self::Item {
                let mut frame = self.source.pull();
                for sample in frame.content.iter_mut() {
                    *sample *= self.gain;
                }
                frame
            }
        }

        let &SocketRef (ref node_ref, socket_idx) = self.source.as_ref().unwrap();
        let mut node_ref = node_ref.write().unwrap();
        let source = node_ref.try_get_source(socket_idx).unwrap();

        let audio_source = InnerSource {
            gain: self.gain,
            source,
        };
        let audio_source: Box<dyn Source<Item = AudioPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}

struct CombineNode {
    source1: Option<SocketRef>,
    source2: Option<SocketRef>,
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
    fn connect(
        &mut self,
        to_socket: SocketIdx,
        from_node: NodeRef,
        from_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        match to_socket {
            0 => {
                self.source1 = Some(SocketRef(from_node, from_socket));
                Ok(())
            }
            1 => {
                self.source2 = Some(SocketRef(from_node, from_socket));
                Ok(())
            }
            _ => Err(SocketConnectionError::NoSuchSocket),
        }
    }

    fn disconnect(&mut self, socket: SocketIdx) {
        match socket {
            0 => self.source1 = None,
            1 => self.source2 = None,
            _ => (),
        }
    }

    fn as_source(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError> {
        if from_socket != 0 {
            return Err(SocketConnectionError::NoSuchSocket);
        }

        struct InnerSource {
            source1: Box<dyn Source<Item = AudioPcm>>,
            source2: Box<dyn Source<Item = AudioPcm>>,
        }
        impl Source for InnerSource {
            type Item = AudioPcm;

            fn pull(&mut self) -> Self::Item {
                let frame1 = self.source1.pull();
                let frame2 = self.source2.pull();
                AudioPcm {
                    sample_rate: frame1.sample_rate,
                    content: frame1
                        .content
                        .iter()
                        .zip(frame2.content.iter())
                        .map(|(a, b)| *a + *b)
                        .collect(),
                }
            }
        }

        let &SocketRef (ref node_ref, socket_idx) = self.source1.as_ref().unwrap();
        let mut node_ref = node_ref.write().unwrap();
        let source1 = node_ref.try_get_source(socket_idx).unwrap();

        let &SocketRef (ref node_ref, socket_idx) = self.source2.as_ref().unwrap();
        let mut node_ref = node_ref.write().unwrap();
        let source2 = node_ref.try_get_source(socket_idx).unwrap();

        let audio_source = InnerSource {
            source1,
            source2,
        };
        let audio_source: Box<dyn Source<Item = AudioPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}

struct WAVExporter {
    file: PathBuf,
    sample_rate: usize,
    source: Option<SocketRef>,
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
        self.source = Some(SocketRef(from_node, from_socket));
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

        let &SocketRef (ref node_ref, socket_idx) = self.source.as_ref().unwrap();
        let mut node_ref = node_ref.write().unwrap();
        let mut source = node_ref.try_get_source(socket_idx).unwrap();

        let audio_pcm: AudioPcm = source.pull();
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
