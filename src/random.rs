#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_random_get();
}

pub fn get() {
    unsafe {
        __wasi_random_get();
    }
}
