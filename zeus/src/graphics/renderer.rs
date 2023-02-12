use crate::graphics::index_buffer::IndexBuffer;
use crate::graphics::shader::Shader;
use crate::graphics::vertex_array::VertexArray;
use core::ptr::null;
use gl;

pub unsafe fn gl_clear_errors() {
    while gl::GetError() != gl::NO_ERROR {}
}

pub unsafe fn gl_check_error(func_name: &str, file_name: &str, line: u32) {
    loop {
        let error = gl::GetError();
        if error == 0 {
            break;
        }
        println!(
            "open_gl Error: {}, func: {}, file_name: {}, line: {}",
            error, func_name, file_name, line
        );
        debug_assert!(false);
    }
}

#[macro_export]
macro_rules! log_gl_error {
    ($func:expr) => {
        unsafe {
            gl_clear_errors();
            $func;
            gl_check_error(stringify!($func), file!(), line!())
        }
    };

    ($stmt:stmt) => {
        unsafe { gl_clear_errors() };
        let name = stringify!($stmt);
        $stmt
        unsafe { gl_check_error(name, file!(), line!()) };
    };
}

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, vertex_array: &VertexArray, index_buffer: &IndexBuffer, shader: &Shader) {
        shader.bind();
        vertex_array.bind();
        index_buffer.bind();
        log_gl_error!(gl::DrawElements(
            gl::TRIANGLES,
            index_buffer.count,
            gl::UNSIGNED_INT,
            null()
        ));
    }

    pub fn clear(&self) {
        log_gl_error!(gl::Clear(gl::COLOR_BUFFER_BIT));
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        log_gl_error!(gl::ClearColor(red, green, blue, alpha));
    }
}
