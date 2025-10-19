use overtone::transformer::{Node, NodeRef, SocketConnectionError, SocketIdx, SocketRef, Source};
use std::any::Any;
use crate::audio::{AudioChunkPcm, AudioPcm};

pub struct GainNode {
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
            source: Box<dyn Source<Item = AudioChunkPcm>>,
        }
        impl Source for InnerSource {
            type Item = AudioChunkPcm;

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
        let audio_source: Box<dyn Source<Item = AudioChunkPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}