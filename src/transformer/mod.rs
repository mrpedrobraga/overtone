//! # Production Setups
//!
//! A "Production" uses the [`Element`]s of an arrangement to do things.
//!
//! If this description sounds generic, it's because productions can do all sorts of things.

use std::sync::{Arc, RwLock, Weak};
use {crate::renderer::RenderExporter, std::any::Any};
use crate::renderer::ExportError;

pub type SocketIdx = usize;
pub type FormatName = String;
pub type NodeRef = Arc<RwLock<dyn Node>>;
pub type NodeWeakRef = Weak<RwLock<dyn Node>>;

/// The main struct of this module.
pub struct ProductionSetup {
    connections: Vec<NodeConnection>,

    /// The exporters in the production setup.
    ///
    /// These are the things that transform
    /// productions into files on disk.
    ///
    /// Exporters are also responsible for Previewing.
    exporters: Vec<Box<dyn RenderExporter>>,
}

pub trait Node {
    fn as_exporter(&self) -> Option<&dyn RenderExporter> {
        None
    }
}

impl ProductionSetup {
    pub fn new() -> Self {
        ProductionSetup {
            connections: Vec::new(),
            exporters: Vec::new(),
        }
    }

    pub fn try_connect(
        &mut self,
        node_a: &NodeRef,
        outgoing_socket: SocketIdx,
        node_b: &NodeRef,
        incoming_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        Ok(self.connections.push(NodeConnection {
            producer: (node_a.clone(), outgoing_socket),
            receiver: (incoming_socket, node_b.clone()),
        }))
    }

    pub fn export(&self, from_node: NodeRef) -> Result<(), ExportError> {
        let guard = from_node.read().unwrap();
        if let Some(exporter) = guard.as_exporter() {
            exporter.export()
        } else {
            Err(ExportError::Generic)
        }
    }
}

pub struct NodeConnection {
    producer: (NodeRef, SocketIdx),
    receiver: (SocketIdx, NodeRef),
}

pub struct SocketInfo {
    index: SocketIdx,
    format: FormatName,
}

pub enum SocketConnectionError {
    /// Connection refused because the format of the incoming signal
    /// is not compatible with this socket.
    IncorrectFormat {
        expected: Option<Box<dyn Iterator<Item = FormatName>>>,
    },
}

pub trait SignalFrame {
    fn get_format_name(&self) -> FormatName;

    /// Returns this render result as an [`std::any::Any`];
    ///
    /// Look, just write `self` in there.
    ///
    /// The only reason why this isn't a given method because you're not converting the
    /// [`SignalFrame`], this method will be called on each concrete
    /// type you have.
    fn as_any(&self) -> &dyn Any;

    /// Attempts to parse this mysterious [`SignalFrame`]
    /// into some other type.
    fn try_as<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}
