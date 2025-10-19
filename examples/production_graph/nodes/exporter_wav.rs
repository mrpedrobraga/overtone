use std::path::{Path, PathBuf};
use overtone::transformer::{ExportError, Node, NodeRef, Sink, SocketConnectionError, SocketIdx, SocketRef, Source};
use std::any::Any;
use crate::audio::AudioPcm;

pub struct WAVExporter {
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

        let location = location.join(format!("export-{}.wav", chrono::Utc::now().to_string()));

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