use overtone::transformer::{Node, NodeRef, SocketConnectionError, SocketIdx, SocketRef, Source};
use std::any::Any;
use crate::audio::{AudioChunkPcm, AudioPcm};

pub struct CombineNode {
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
            source1: Box<dyn Source<Item = AudioChunkPcm>>,
            source2: Box<dyn Source<Item = AudioChunkPcm>>,
        }
        impl Source for InnerSource {
            type Item = AudioChunkPcm;

            fn pull(&mut self) -> Self::Item {
                let frame1 = self.source1.pull();
                let frame2 = self.source2.pull();
                AudioChunkPcm {
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
        let audio_source: Box<dyn Source<Item = AudioChunkPcm>> = Box::new(audio_source);
        Ok(Box::new(audio_source))
    }
}