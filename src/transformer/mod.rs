//! # Production Setups
//!
//! A "Production" uses the [`Element`]s of an arrangement to do things.
//!
//! If this description sounds generic, it's because productions can do all sorts of things.
//!
//! ![](https://media.discordapp.net/attachments/1007414627090583684/1428822879759368202/SPOILER_graph_effects.png)

use {crate::renderer::RenderExporter, std::any::Any};

pub type SocketIdx = usize;
pub type FormatName = String;

/// The main struct of this module.
pub struct ProductionSetup {
    /// The exporters in the production setup.
    ///
    /// These are the things that transform
    /// productions into files on disk.
    ///
    /// Exporters are also responsible for Previewing.
    exporters: Vec<Box<dyn RenderExporter>>,
}

pub trait Node {
    /// Attempts to connect
    fn request_connect(
        &mut self,
        my_out_socket: SocketIdx,
        its_in_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError>;
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
