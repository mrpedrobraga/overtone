unsafe extern "C" {
    pub fn start();
}

#[test]
fn test_start_function() {
    unsafe {
        start();
    }
}
