use std::marker::PhantomData;
use crate::graph::{Node, SocketData};

pub struct NumSource {
    pub value: f64,
}

impl Node for NumSource {
    fn bind_parameters(&self, parameters: &mut dyn Iterator<Item=*mut u8>) -> Box<dyn FnMut()> {
        let value = self.value;
        let out = as_output::<f64>(parameters.next().unwrap());

        Box::new(move || {
            *out = value;
        })
    }

    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            _ => None,
        }
    }

    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            _ => None,
        }
    }
}

pub struct Sum;

impl Node for Sum {
    fn bind_parameters(&self, parameters: &mut dyn Iterator<Item=*mut u8>) -> Box<dyn FnMut()> {
        let in1 = as_input::<f64>(parameters.next().unwrap());
        let in2 = as_input::<f64>(parameters.next().unwrap());
        let out = as_output::<f64>(parameters.next().unwrap());

        Box::new(move || *out = *in1 + *in2)
    }

    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            1 => Some(SocketData::new::<f64>()),
            _ => None,
        }
    }

    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            _ => None,
        }
    }
}

pub struct Double;

impl Node for Double {
    fn bind_parameters(&self, parameters: &mut dyn Iterator<Item=*mut u8>) -> Box<dyn FnMut()> {
        let in1 = as_input::<f64>(parameters.next().unwrap());
        let out = as_output::<f64>(parameters.next().unwrap());

        Box::new(move || *out = *in1 * 2.0)
    }

    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            _ => None,
        }
    }

    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            _ => None,
        }
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
    fn bind_parameters(&self, parameters: &mut dyn Iterator<Item=*mut u8>) -> Box<dyn FnMut()> {
        let in1 = as_input::<f64>(parameters.next().unwrap());

        Box::new(move || {
            print!("{};", *in1);
        })
    }

    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            _ => None,
        }
    }

    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            _ => None,
        }
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