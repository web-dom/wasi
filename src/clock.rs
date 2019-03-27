#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_clock_res_get();
}

pub fn res_get() {
    unsafe {
        __wasi_clock_res_get();
    }
}

extern "C" {
    fn __wasi_clock_time_get();
}

pub fn time_get() {
    unsafe {
        __wasi_clock_time_get();
    }
}
