use std::os::raw::c_void;

use gl::types::*;

pub struct Buffer {

    buffer_id: GLuint,
    component_count: GLuint,

}

impl Buffer {
    pub fn vertex_buffer(&mut self, data: *const GLfloat, count: GLsizeiptr, component_count: GLuint) {
        self.component_count = component_count;

        unsafe {
            gl::GenBuffers(1, &mut self.buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
            let size = count * size_of::<GLfloat>() as GLsizeiptr ;
            gl::BufferData(gl::ARRAY_BUFFER, size, data as *const _, gl::STATIC_DRAW);
            // Unbindbuffer;
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        }

    }
}