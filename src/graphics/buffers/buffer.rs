use std::os::raw::c_void;

use gl::types::*;

pub struct Buffer {
    pub buffer_id: GLuint,
    pub component_count: GLuint,
}

impl Buffer {
    pub fn from(data: &[f32], count: GLsizeiptr, component_count: GLuint) -> Self {
        let component_count = component_count;
        let mut buffer_id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            let size = count * size_of::<GLfloat>() as GLsizeiptr;
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            // Unbindbuffer;
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self {
            buffer_id,
            component_count,
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

// impl Drop for Buffer {
//     fn drop(&mut self) {
//         // causes SOME MEMORY PROBLEM
//         println!("called");
//         // unsafe {
//         //     gl::DeleteBuffers(1, self.buffer_id as *const _);
//         // }
//     }
// }
