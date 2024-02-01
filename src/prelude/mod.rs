/// Declares a new Overtone [`Plugin`] instance, with everything it might do.
#[macro_export]
macro_rules! overtone_plugin {
    ( $e: expr ) => {
        #[no_mangle]
        pub fn get_overtone_plugin() -> Box<dyn $crate::plugin::Plugin> { $e }
    }
}

pub use maplit::hashmap;
pub use maplit::hashset;