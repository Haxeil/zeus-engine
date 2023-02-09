use crate::graphics::renderer::*;
use crate::log_gl_error;
use std::os::raw::c_void;

pub struct VertexBuffer {
    renderer_id: u32,
}

impl VertexBuffer {
    pub fn new() -> Self {
        Self { renderer_id: 0 }
    }
    pub fn construct(mut self, data: *const c_void, size: isize) -> Self {
        unsafe {
            log_gl_error!(gl::GenBuffers(1, &mut self.renderer_id));
            log_gl_error!(gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id));
            log_gl_error!(gl::BufferData(
                gl::ARRAY_BUFFER,
                size,
                data,
                gl::STATIC_DRAW,
            ));
        }
        self
    }

    pub fn bind(&self) {
        unsafe {
            log_gl_error!(gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id));
        }
    }

    pub fn unbind() {
        unsafe {
            log_gl_error!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            log_gl_error!(gl::DeleteBuffers(1, &self.renderer_id));
        }
    }
}
