#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_sched_yield();
}

pub fn r#yield() {
    unsafe {
        __wasi_sched_yield();
    }
}
