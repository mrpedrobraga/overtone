//! # Production Setups
//!
//! A "Production" uses the [`Element`]s of an arrangement to do things.
//!
//! If this description sounds generic, it's because productions can do all sorts of things.

use std::sync::{Arc, RwLock, Weak};
use {std::any::Any};
use crate::IOError;

pub type SocketIdx = usize;
pub type FormatName = String;
pub type NodeRef = Arc<RwLock<dyn Node>>;
pub type NodeWeakRef = Weak<RwLock<dyn Node>>;

/// The main struct of this module.
///
/// This is a graph that contains interconnected nodes.
pub struct ProductionSetup {
    nodes: Vec<NodeRef>
}

/// An element of a production setup.
/// These can be connected with one another,
/// forming a DAG.
pub trait Node {
    /// Attempts to connect an incoming signal to one of this node's sockets.
    fn connect(&mut self, to_socket: SocketIdx, from_node: NodeRef, from_socket: SocketIdx) -> Result<(), SocketConnectionError>;
    /// Disconnects whatever is connected to the socket at the given index.
    fn disconnect(&mut self, socket: SocketIdx);
    /// Attempts to thread a source, pulling it from one of the output sockets;
    fn as_source(&mut self, from_out_socket: SocketIdx) -> Result<Box<dyn Any>, SocketConnectionError>;
    /// Casts this node to a [`Sink`], if possible.
    fn as_sink(&mut self) -> Option<&mut dyn Sink> {
        None
    }
}

impl dyn Node {
    /// Attempts to cast one of this node's output sockets into a `dyn Source<Item>`,
    /// that will be stored by a receiver node, allowing it to pull `Item`s.
    pub fn try_get_source<Item: 'static>(&mut self, from_socket: SocketIdx) -> Result<Box<dyn Source<Item = Item>>, SocketConnectionError> {
        let source = self.as_source(from_socket)?;
        let item = source.downcast::<Box<dyn Source<Item = Item>>>().unwrap();
        Ok(*item)
    }
}

pub trait Source {
    type Item;
    fn pull(&mut self) -> Self::Item;
}

pub struct SocketRef(pub NodeRef, pub SocketIdx);

#[derive(Debug)]
pub enum SocketConnectionError {
    /// No socket at the given index.
    NoSuchSocket,
    /// Connection refused because the format of the incoming signal
    /// is not compatible with this socket.
    IncorrectFormat {
        expected: Option<Vec<String>>,
    },
}

/// Trait for something that can be passed around by [`Node`]s,
/// hidden behind a `dyn Value`.
pub trait Value {
    /// Returns a string that identifies the type
    /// hidden behind this `dyn Value`.
    ///
    /// I don't use TypeId because type ids change,
    /// can't be serialized, and, strings are neat to remember.
    fn get_format_name(&self) -> FormatName;

    /// Returns this render result as an [`std::any::Any`];
    ///
    /// Look, just write `self` in there.
    ///
    /// The only reason why this isn't a given method because you're not converting the
    /// [`Value`], this method will be called on each concrete
    /// type you have.
    fn as_any(&self) -> &dyn Any;

    /// Attempts to parse this mysterious [`Value`]
    /// into some other type.
    fn try_as<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

/// Trait represents a node that can drive the processing of a graph
/// by pulling on it.
pub trait Sink: Node {
    /// Exhausts the graph by pulling values
    /// and exports them to possibly a file.
    fn drain(&mut self) -> Result<(), ExportError>;
}

/// An error that occurred during an export.
#[derive(Debug)]
pub enum ExportError {
    IO(std::io::Error),
}