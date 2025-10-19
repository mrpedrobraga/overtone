use overtone::transformer::{Node, NodeRef, SocketConnectionError, SocketIdx, Source};
use std::any::Any;
use crate::audio::{AudioChunkPcm, AudioPcm};

pub struct WaveGenerator {
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
            type Item = AudioChunkPcm;

            fn pull(&mut self) -> Self::Item {
                let whole = AudioPcm::example(self.frequency);
                AudioChunkPcm { content: whole.content }
            }
        }
        let audio_source = InnerSource {
            frequency: self.frequency,
        };
        let audio_source: Box<dyn Source<Item = AudioChunkPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}