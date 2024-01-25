/// Trait for a possible output from a renderer.
/// It'll be used by an exporter to preview + write files.
pub trait RenderResult {
    /// Returns the identifier of this render format, which
    /// will serve as a dynamic check of whether two endpoints which
    /// require a [`RenderResult`] can connect.
    ///
    /// This is still just a light check for convenience, and retrieving
    /// a value from this still produces an Option of the underlying type.
    fn get_format_id(&self) -> String;

    fn as_any(&self) -> &dyn std::any::Any;
}

/// Attempts to reinterpret a [`RenderResult`] as a concrete type.
pub fn try_reinterpret<T: 'static>(render_result: &dyn RenderResult) -> Option<&T> {
    render_result.as_any().downcast_ref::<T>()
}
