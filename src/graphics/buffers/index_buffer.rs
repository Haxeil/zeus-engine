
use gl::types::*;

pub struct IndexBuffer {

    buffer_id: GLuint,
    count: GLsizeiptr,

}

impl IndexBuffer {
    pub fn from(data: *const GLushort, count: GLsizeiptr) -> Self {
        let count = count;
        let mut buffer_id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer_id);
            let size = count * size_of::<GLushort>() as GLsizeiptr ;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data as *const _, gl::STATIC_DRAW);
            // Unbindbuffer;
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Self {
            buffer_id,
            count
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        }

    }
}