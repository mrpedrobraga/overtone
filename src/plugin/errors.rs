use crate::api::errors::OvertoneApiError;

#[derive(Debug)]
pub enum PluginError {
    /// Plugin was already loaded and won't load again.
    PluginAlreadyLoaded(),
    /// A plugin, which was referred by this id, does not exist.
    MissingPlugin(String),
    /// Library couldn't be loaded for some reason.
    LibraryNotFound(libloading::Error),
    /// Library was loaded but is not an Overtone plugin.
    LibraryIsNotOvertonePlugin(),
}

impl From<PluginError> for OvertoneApiError {
    fn from(value: PluginError) -> Self {
        OvertoneApiError::PluginError(value)
    }
}