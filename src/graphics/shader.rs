use std::{ffi::CStr, ptr};

use crate::utils;
use gl::{types::*, FALSE};
use utils::file_utils::read_file;

pub struct Shader {
    pub shader: GLuint,
}

impl Shader {
    pub fn from(vertex_path: &str, fragment_path: &str) -> Shader {
        Shader {
            shader: load(vertex_path, fragment_path),
        }
    }
}

impl Shader {
    #[inline]
    fn enable(&mut self) {
        unsafe {
            gl::UseProgram(self.shader);
        }
    }

    #[inline]
    fn disable(&mut self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

fn load(vertex_path: &str, fragment_path: &str) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);

        // VERTEX SHADER;
        gl::ShaderSource(vertex, 1, &read_file(vertex_path).as_ptr(), ptr::null());
        gl::CompileShader(vertex);

        let mut result: GLint = -1;
        gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut result);

        if result == 0 {
            let mut length: GLint = 0;
            gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut length);

            let mut buffer = vec![0u8; length as usize];
            gl::GetShaderInfoLog(vertex, length, ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);

            let msg = std::str::from_utf8(&buffer)
                .unwrap_or("Failed to read shader info log")
                .to_string();

            println!("Vertex shader compilation error: {}", msg);
            gl::DeleteShader(vertex);
            return 0;

        }

        // FRAGMENT SHADER:

        gl::ShaderSource(fragment, 1, &read_file(fragment_path).as_ptr(), ptr::null());
        gl::CompileShader(fragment);

        let mut result: GLint = -1;
        gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut result);

        if result == 0 {
            let mut length: GLint = 0;
            gl::GetShaderiv(fragment, gl::INFO_LOG_LENGTH, &mut length);

            let mut buffer = vec![0u8; length as usize];
            gl::GetShaderInfoLog(fragment, length, ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);

            let msg = std::str::from_utf8(&buffer)
                .unwrap_or("Failed to read shader info log")
                .to_string();

            println!("Fragment shader compilation error: {}", msg);
            gl::DeleteShader(fragment);

            return 0;

        }

        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);

        gl::LinkProgram(program);
        gl::ValidateProgram(program);

        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        return program;
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader);
        }
    }
}