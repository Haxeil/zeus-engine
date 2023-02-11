use gl;
use std::alloc::{alloc, Layout};
use std::error::Error;
use std::ffi::CStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::c_char;
use std::ptr::null;

use crate::graphics::renderer::*;
use crate::log_gl_error;

pub struct ShaderSource {
    pub vertex_shader: String,
    pub fragment_shader: String,
}

impl ShaderSource {
    pub fn new() -> Self {
        Self {
            vertex_shader: String::new(),
            fragment_shader: String::new(),
        }
    }
}

pub struct Shader {
    renderer_id: u32,
    file_path: String,
}

impl Shader {
    pub fn new(file_path: &str) -> Self {
        Self {
            renderer_id: 0,
            file_path: file_path.to_string(),
        }
    }

    pub fn construct(mut self) -> Result<Self, Box<dyn Error>> {
        let source = self.parse_shader(&self.file_path)?;
        self.renderer_id = self.create_shader(source.vertex_shader, source.fragment_shader)?;

        Ok(self)
    }

    pub fn bind(&self) {
        log_gl_error!(gl::UseProgram(self.renderer_id))
    }
    pub fn unbind(&self) {
        log_gl_error!(gl::UseProgram(0))
    }

    pub fn set_uniform_4f(&self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        log_gl_error!(gl::Uniform4f(
            self.get_uniform_location(name),
            v0,
            v1,
            v2,
            v3
        ));
    }

    fn get_uniform_location(&self, name: &str) -> gl::types::GLint {
        log_gl_error!(let location = unsafe {gl::GetUniformLocation(self.renderer_id, name.as_ptr() as *const gl::types::GLchar)});
        if location == -1 {
            println!("warning: Uniform {} does not exist", name,);
        }
        location
    }

    fn create_shader(
        &self,
        vertex_shader: String,
        fragment_shader: String,
    ) -> Result<u32, Box<dyn Error>> {
        let program = unsafe { gl::CreateProgram() };
        let vs = self.compile_shader(gl::VERTEX_SHADER, vertex_shader)?;
        let fs = self.compile_shader(gl::FRAGMENT_SHADER, fragment_shader)?;
        // Attach Shaders;
        log_gl_error!(gl::AttachShader(program, vs));
        log_gl_error!(gl::AttachShader(program, fs));
        log_gl_error!(gl::LinkProgram(program));
        log_gl_error!(gl::ValidateProgram(program));
        // Drop Shaders;
        log_gl_error!(gl::DeleteShader(vs));

        Ok(program)
    }

    fn compile_shader(&self, c_type: u32, source: String) -> Result<u32, Box<dyn Error>> {
        let id = unsafe { gl::CreateShader(c_type) };
        // let src = source.as_bytes().as_ptr() as *const *const i8;
        let c_string = alloc::ffi::CString::new(source)?;
        let source = c_string.as_ptr();
        let source = &source as *const *const i8;

        log_gl_error!(gl::ShaderSource(id, 1, source, null()));
        log_gl_error!(gl::CompileShader(id));

        let mut result = 0;
        log_gl_error!(gl::GetShaderiv(
            id,
            gl::COMPILE_STATUS,
            &mut result as *mut i32
        ));

        if result as u8 == gl::FALSE {
            let mut length: i32 = 0;
            log_gl_error!(gl::GetShaderiv(
                id,
                gl::INFO_LOG_LENGTH,
                &mut length as *mut _
            ));
            let layout = Layout::from_size_align(length.try_into()?, 1)?;
            let message: *mut c_char = unsafe { alloc(layout) as *mut i8 };
            log_gl_error!(gl::GetShaderInfoLog(
                id,
                length,
                &mut length as *mut _,
                message
            ));
            println!("Failed to compile: {}", unsafe {
                CStr::from_ptr(message).to_str()?
            });

            log_gl_error!(gl::DeleteProgram(id));
            return Ok(0);
        }
        Ok(id)
    }

    fn parse_shader(&self, path: &str) -> Result<ShaderSource, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut shaders = ShaderSource::new();
        let reader = BufReader::new(file);
        let mut is_fragment_shader = false;

        for line in reader.lines() {
            let line = line?;
            if line.ends_with("vertex") {
                continue;
            }

            if line.ends_with("fragment") {
                is_fragment_shader = true;
                continue;
            }

            if is_fragment_shader {
                shaders.fragment_shader.push_str(line.trim());
                shaders.fragment_shader.push_str("\n");
            } else {
                shaders.vertex_shader.push_str(line.trim());
                shaders.vertex_shader.push_str("\n");
            }
        }

        Ok(shaders)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        log_gl_error!(gl::DeleteProgram(self.renderer_id));
    }
}
