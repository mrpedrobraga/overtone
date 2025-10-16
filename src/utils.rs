use std::rc::Rc;

/// Trait that allows extracting some metadata from foreign types.
pub trait Info {
    fn get_name(&self) -> &str;
}

pub type RefStr = Rc<str>;
pub type DependencyId = String;