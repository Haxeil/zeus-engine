use gl::types::*;

use super::{buffer::Buffer, index_buffer};

pub struct VertexArray<'a> {
    array_id: GLuint,
    buffers: Vec<&'a mut Buffer>,
}

impl VertexArray<'_> {

    pub fn new() -> Self {
        let mut vertex_array = Self {
            array_id: 0,
            buffers: vec![],
        };

        unsafe  {
            gl::GenVertexArrays(1, &mut vertex_array.array_id);
        }

        vertex_array
    }


}

impl VertexArray<'_> {
    pub fn add_buffer(&mut self, buffer: &mut Buffer, index: GLuint) {
        self.bind();

        buffer.bind();

        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(index, buffer.component_count as i32, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
        }

        buffer.unbind();
        self.unbind();


    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.array_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
} 

// impl Drop for VertexArray<'_> {
//     fn drop(&mut self) {

//         unsafe {
//             for buffer in &self.buffers {
//                 gl::DeleteBuffers(1, buffer.buffer_id as *const _);
//             }
//         }
//         self.buffers.clear();

//         unsafe {
//             gl::DeleteBuffers(1, self.array_id as *const _);
//         }

//     }
// }