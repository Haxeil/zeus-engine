use crate::graphics::renderer::*;
use crate::log_gl_error;
use std::mem::size_of;
use std::os::raw::c_void;

pub struct IndexBuffer {
    renderer_id: u32,
    count: i32,
}

impl IndexBuffer {
    pub fn new() -> Self {
        Self {
            renderer_id: 0,
            count: 0,
        }
    }

    pub fn construct(mut self, data: *const c_void, count: i32) -> Self{
        self.count = count;
        unsafe {
            log_gl_error!(gl::GenBuffers(1, &mut self.renderer_id));
            log_gl_error!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id));
            log_gl_error!(gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            count as isize * size_of::<gl::types::GLuint>() as isize,
            data.into(),
            gl::STATIC_DRAW,
        ));
        }
        self

    }

    pub fn bind(&self) {
        unsafe {
            log_gl_error!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id));

        }
    }

    pub fn unbind() {
        unsafe {
            log_gl_error!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));

        }
    }

    #[inline]
    pub fn get_count(&self) -> i32 {
        self.count
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            log_gl_error!(gl::DeleteBuffers(1, &self.renderer_id));

        }
    }
}

