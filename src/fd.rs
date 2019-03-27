#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_fd_advise();
}

pub fn advise() {
    unsafe {
        __wasi_fd_advise();
    }
}

extern "C" {
    fn __wasi_fd_allocate();
}

pub fn allocate() {
    unsafe {
        __wasi_fd_allocate();
    }
}

extern "C" {
    fn __wasi_fd_close();
}

pub fn close() {
    unsafe {
        __wasi_fd_close();
    }
}

extern "C" {
    fn __wasi_fd_datasync();
}

pub fn datasync() {
    unsafe {
        __wasi_fd_datasync();
    }
}

extern "C" {
    fn __wasi_fd_fdstat_get();
}

pub fn fdstat_get() {
    unsafe {
        __wasi_fd_fdstat_get();
    }
}

extern "C" {
    fn __wasi_fd_fdstat_set_flags();
}

pub fn fdstat_set_flags() {
    unsafe {
        __wasi_fd_fdstat_set_flags();
    }
}

extern "C" {
    fn __wasi_fd_fdstat_set_rights();
}

pub fn fdstat_set_rights() {
    unsafe {
        __wasi_fd_fdstat_set_rights();
    }
}

extern "C" {
    fn __wasi_fd_filestat_get();
}

pub fn filestat_get() {
    unsafe {
        __wasi_fd_filestat_get();
    }
}

extern "C" {
    fn __wasi_fd_filestat_set_size();
}

pub fn filestat_set_size() {
    unsafe {
        __wasi_fd_filestat_set_size();
    }
}

extern "C" {
    fn __wasi_fd_filestat_set_times();
}

pub fn filestat_set_times() {
    unsafe {
        __wasi_fd_filestat_set_times();
    }
}

extern "C" {
    fn __wasi_fd_pread();
}

pub fn pread() {
    unsafe {
        __wasi_fd_pread();
    }
}

extern "C" {
    fn __wasi_fd_prestat_get();
}

pub fn prestat_get() {
    unsafe {
        __wasi_fd_prestat_get();
    }
}

extern "C" {
    fn __wasi_fd_prestat_dir_name();
}

pub fn prestat_dir_name() {
    unsafe {
        __wasi_fd_prestat_dir_name();
    }
}

extern "C" {
    fn __wasi_fd_pwrite();
}

pub fn pwrite() {
    unsafe {
        __wasi_fd_pwrite();
    }
}

extern "C" {
    fn __wasi_fd_read();
}

pub fn read() {
    unsafe {
        __wasi_fd_read();
    }
}

extern "C" {
    fn __wasi_fd_readdir();
}

pub fn readdir() {
    unsafe {
        __wasi_fd_readdir();
    }
}

extern "C" {
    fn __wasi_fd_renumber();
}

pub fn renumber() {
    unsafe {
        __wasi_fd_renumber();
    }
}

extern "C" {
    fn __wasi_fd_seek();
}

pub fn seek() {
    unsafe {
        __wasi_fd_seek();
    }
}

extern "C" {
    fn __wasi_fd_sync();
}

pub fn sync() {
    unsafe {
        __wasi_fd_sync();
    }
}

extern "C" {
    fn __wasi_fd_tell();
}

pub fn tell() {
    unsafe {
        __wasi_fd_tell();
    }
}

extern "C" {
    fn __wasi_fd_write();
}

pub fn write() {
    unsafe {
        __wasi_fd_write();
    }
}
