
use gl::types::*;

#[derive(Clone)]
pub struct IndexBuffer {

    buffer_id: GLuint,
    pub count: GLsizeiptr,

}

impl IndexBuffer {
    pub fn from(data: &[GLushort], count: isize) -> Self {
        let count = count;
        let mut buffer_id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer_id);
            let size = count * size_of::<GLushort>() as GLsizeiptr ;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data.as_ptr() as *const _, gl::STATIC_DRAW);
            // Unbindbuffer;
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Self {
            buffer_id,
            count
        }
    }

    pub fn bind(&self) {
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