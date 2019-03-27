#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn __wasi_path_create_directory();
}

pub fn create_directory() {
    unsafe {
        __wasi_path_create_directory();
    }
}

extern "C" {
    fn __wasi_path_filestat_get();
}

pub fn filestat_get() {
    unsafe {
        __wasi_path_filestat_get();
    }
}

extern "C" {
    fn __wasi_path_filestat_set_times();
}

pub fn filestat_set_times() {
    unsafe {
        __wasi_path_filestat_set_times();
    }
}

extern "C" {
    fn __wasi_path_link();
}

pub fn link() {
    unsafe {
        __wasi_path_link();
    }
}

extern "C" {
    fn __wasi_path_open();
}

pub fn open() {
    unsafe {
        __wasi_path_open();
    }
}

extern "C" {
    fn __wasi_path_readlink();
}

pub fn readlink() {
    unsafe {
        __wasi_path_readlink();
    }
}

extern "C" {
    fn __wasi_path_remove_directory();
}

pub fn remove_directory() {
    unsafe {
        __wasi_path_remove_directory();
    }
}

extern "C" {
    fn __wasi_path_rename();
}

pub fn rename() {
    unsafe {
        __wasi_path_rename();
    }
}

extern "C" {
    fn __wasi_path_symlink();
}

pub fn symlink() {
    unsafe {
        __wasi_path_symlink();
    }
}

extern "C" {
    fn __wasi_path_unlink_file();
}

pub fn unlink_file() {
    unsafe {
        __wasi_path_unlink_file();
    }
}
