use std::fmt::Debug;

#[derive(Debug)]
pub struct Arrangement {
    name: String,
    root_fragment: Fragment,
}

#[derive(Debug)]
pub struct Fragment {
    format: FragmentFormatRef,
    content: Box<dyn FragmentContent>,
}

#[derive(Debug)]
pub struct FragmentFormatRef {
    id: &'static str,
}

pub trait FragmentContent: Debug {}
