#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_sock_recv();
}

pub fn recv() {
    unsafe {
        __wasi_sock_recv();
    }
}

extern "C" {
    fn __wasi_sock_send();
}

pub fn send() {
    unsafe {
        __wasi_sock_send();
    }
}

extern "C" {
    fn __wasi_sock_shutdown();
}

pub fn shutdown() {
    unsafe {
        __wasi_sock_shutdown();
    }
}
