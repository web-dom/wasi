#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_poll_oneoff();
}

pub fn oneoff() {
    unsafe {
        __wasi_poll_oneoff();
    }
}
