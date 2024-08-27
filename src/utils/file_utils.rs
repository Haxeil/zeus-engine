use std::{
    error::Error,
    ffi::{c_char, CString},
    fs,
};

pub fn read_file(name: &str) -> CString {
    let content = fs::read_to_string(name).expect("Failed to read shader file");
    CString::new(content).expect("Failed to convert to C string")
}