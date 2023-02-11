use crate::graphics::vertex_buffer::VertexBuffer;
use crate::graphics::vertex_buffer_layout::VertexBufferLayout;
use crate::log_gl_error;
use crate::graphics::renderer::*;


pub struct VertexArray {
    renderer_id: u32,
}

impl VertexArray {
    pub fn new() -> Self {


        Self {
            renderer_id: 0
        }
    }

    pub fn construct(mut self) -> Self {
        log_gl_error!(gl::GenVertexArrays(1, &mut self.renderer_id));
        self
    }

    pub fn add_buffer(&self, vb: &VertexBuffer, layout: &VertexBufferLayout) {
        self.bind();
        vb.bind();
        let mut offset = 0_usize;

        for (i, elm) in layout.elements.iter().enumerate() {

            log_gl_error!(gl::EnableVertexAttribArray(i as u32));
            log_gl_error!(gl::VertexAttribPointer(
            i as u32,
            elm.count as i32,
            elm.v_type.get_value(),
            elm.normalized as u8,
            layout.stride as i32,
            offset as *const _
            ));
            offset += elm.count as usize * elm.size_of_current_type();
        }


    }

    pub fn bind(&self) {
        log_gl_error!(gl::BindVertexArray(self.renderer_id))
    }

    pub fn unbind() {
        log_gl_error!(gl::BindVertexArray(0))

    }
}


impl Drop for VertexArray {
    fn drop(&mut self) {
        log_gl_error!(gl::DeleteVertexArrays(1, &self.renderer_id));
    }
}