#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_environ_get();
}

pub fn get() {
    unsafe {
        __wasi_environ_get();
    }
}

extern "C" {
    fn __wasi_environ_sizes_get();
}

pub fn sizes_get() {
    unsafe {
        __wasi_environ_sizes_get();
    }
}
