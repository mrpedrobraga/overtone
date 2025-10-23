use std::marker::PhantomData;
use crate::graph::Node;

pub struct NumSource {
    pub value: f64,
}

impl Node for NumSource {
    fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()> {
        let value = self.value;
        let out = outputs[0];
        let out = as_output::<f64>(out);

        Box::new(move || {
            *out = value;
        })
    }
}

pub struct Sum;

impl Node for Sum {
    fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()> {
        let in1 = as_input::<f64>(inputs[0]);
        let in2 = as_input::<f64>(inputs[1]);
        let out = as_output::<f64>(outputs[0]);
        Box::new(move || *out = *in1 + *in2)
    }
}

pub struct Double;

impl Node for Double {
    fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()> {
        let in1 = as_input::<f64>(inputs[0]);
        let out = as_output::<f64>(outputs[0]);
        Box::new(move || *out = *in1 * 2.0)
    }
}

// #[derive(Default)]
// pub struct Split<T> {
//     marker: PhantomData<T>,
// }
// impl<T: Clone + std::fmt::Debug + 'static> Node for Split<T> {
//     fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()> {
//         let input = as_input::<T>(inputs[0]);
//         let out1 = as_output::<T>(outputs[0]);
//         let out2 = as_output::<T>(outputs[1]);
//
//         Box::new(|| {
//             let value = input.clone();
//             *out1 = value.clone();
//             *out2 = value;
//         })
//     }
// }

pub struct YellNum;

impl Node for YellNum {
    fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()> {
        let in1 = as_input::<f64>(inputs[0]);
        Box::new(move || {
            print!("{};", *in1);
        })
    }
}

#[inline]
pub fn as_input<'a, T>(ptr: *const u8) -> &'a T {
    unsafe { &*ptr.cast::<T>() }
}

#[inline]
pub fn as_output<'a, T>(ptr: *mut u8) -> &'a mut T {
    unsafe { &mut *ptr.cast::<T>() }
}