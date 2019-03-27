#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_args_get(input_start: i32);
}

pub fn get() {
    unsafe {
        __wasi_args_get(0);
    }
}

extern "C" {
    fn __wasi_args_sizes_get();
}

pub fn sizes_get() {
    unsafe {
        __wasi_args_sizes_get();
    }
}
