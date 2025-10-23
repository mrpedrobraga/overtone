use std::time::Instant;

/// The code above should effectively do this
/// automatically — using values, associated types, traits, etc;
fn handmade_model() {
    let mut edge_data: Vec<u8> = Vec::new();

    edge_data.extend_from_slice(&[0, 0, 0, 0]);
    edge_data.extend_from_slice(&[0, 0, 0, 0]);
    edge_data.extend_from_slice(&[0, 0, 0, 0]);

    let edge_data_ptr = edge_data.as_mut_ptr();

    let mut vertices: Vec<Box<dyn FnMut()>> = Vec::new();

    vertices.push(Box::new(|| {
        proc_a(edge_data_ptr.wrapping_add(0));
    }));
    vertices.push(Box::new(|| {
        proc_b(edge_data_ptr.wrapping_add(8));
    }));
    vertices.push(Box::new(|| {
        proc_c(
            // The variable parameter count will be hard to pull off...
            // There is one parameter for each input or output edge.
            edge_data_ptr.wrapping_add(0),
            edge_data_ptr.wrapping_add(8),
            edge_data_ptr.wrapping_add(12),
        );
    }));

    let amount = 10000;
    let before = Instant::now();
    for _ in 0..amount {
        for process_vertex in vertices.iter_mut() {
            process_vertex();
        }
    }
    println!("After = {:?}", before.elapsed().div_f32(amount as f32));

    let result = as_ref::<f32>(edge_data_ptr.wrapping_add(12));
    dbg!(result);
}

fn proc_a(out0: *mut u8) {
    let out0 = as_mut_ref::<f32>(out0);

    *out0 = 1.0;
}

fn proc_b(out0: *mut u8) {
    let out0 = as_mut_ref::<f32>(out0);

    *out0 = 2.0;
}

fn proc_c(in0: *const u8, in1: *const u8, out0: *mut u8) {
    let in0 = as_ref::<f32>(in0);
    let in1 = as_ref::<f32>(in1);
    let out0 = as_mut_ref::<f32>(out0);

    *out0 = *in0 + *in1;
}

#[inline]
pub fn as_ref<'a, T>(ptr: *const u8) -> &'a T {
    unsafe { &*ptr.cast::<T>() }
}

#[inline]
pub fn as_mut_ref<'a, T>(ptr: *mut u8) -> &'a mut T {
    unsafe { &mut *ptr.cast::<T>() }
}
