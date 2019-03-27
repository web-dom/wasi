#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_proc_exit();
}

pub fn exit() {
    unsafe {
        __wasi_proc_exit();
    }
}

extern "C" {
    fn __wasi_proc_raise();
}

pub fn raise() {
    unsafe {
        __wasi_proc_raise();
    }
}
