use std::{ffi::{CStr, CString}, ptr};

use crate::{mat4::{self, Mat4}, utils, vec2::Vec2, vec3::Vec3, vec4::Vec4};
use gl::{types::*, FALSE};
use utils::file_utils::read_file;

pub struct Shader {
    pub shader_id: GLuint,
}

impl Shader {
    pub fn from(vertex_path: &str, fragment_path: &str) -> Shader {
        Shader {
            shader_id: load(vertex_path, fragment_path),
        }
    }
}

impl Shader {
    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), value);
        }

    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        unsafe {

            gl::Uniform1i(self.get_uniform_location(name), value);
        }
    }

    pub fn set_uniform_2f(&self, name: &str, vector: Vec2) {
        unsafe {

            gl::Uniform2f(self.get_uniform_location(name), vector.x, vector.y);
        }

    }

    pub fn set_uniform_3f(&self, name: &str, vector: Vec3) {
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), vector.x, vector.y, vector.z);
        }
    }

    pub fn set_uniform_4f(&self, name: &str, vector: Vec4) {
        unsafe {

            gl::Uniform4f(self.get_uniform_location(name), vector.x, vector.y, vector.z, vector.w);
        }
    }
    
    pub fn set_uniform_mat4(&self, name: &str, matrix: Mat4) {

        unsafe {
            

            gl::UniformMatrix4fv(self.get_uniform_location(name), 1, gl::FALSE, matrix.elements.as_ptr());
        }


    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        let name = CString::new(name).expect("invalid string");
        unsafe {
            gl::GetUniformLocation(self.shader_id, name.as_ptr())
        }
    
    }

    #[inline]
    pub fn enable(&mut self) {
        unsafe {
            gl::UseProgram(self.shader_id);
        }
    }

    #[inline]
    pub fn disable(&mut self) {
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
            gl::DeleteProgram(self.shader_id);
        }
    }
}