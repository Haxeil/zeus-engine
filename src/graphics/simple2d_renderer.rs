use std::{cell::{Cell, RefCell, RefMut}, collections::VecDeque, os::raw::c_void, rc::Rc};

use crate::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

use super::{renderable2d::Renderable2D, renderer::{Render, Renderer}};
use gl::types::*;
pub struct Simple2dRenderer<'a> {
    pub renderer: Renderer<'a>,
}

impl Simple2dRenderer<'_> {
    pub fn new() -> Self {
        Self {
           renderer: Renderer::new(),
        }
    }
}



impl<'a> Render<'a> for Simple2dRenderer<'a> {


    fn submit(&mut self, renderable2d: Rc<RefCell<Renderable2D<'a>>>) {
        self.renderer.render_queue.push_back(renderable2d);
    }

    fn flush(&mut self) {
        while !self.renderer.render_queue.is_empty() {
            if let Some(renderable) = self.renderer.render_queue.front_mut() {

                let renderable = renderable.borrow_mut();

                renderable.vertex_array.bind();
                
                renderable.index_buffer.bind();

                renderable.shader.set_uniform_mat4("ml_matrix", Mat4::translation(&renderable.position));
                unsafe {
                    gl::DrawElements(gl::TRIANGLES, renderable.index_buffer.count as i32, gl::UNSIGNED_SHORT, 0  as *const c_void);

                }

                renderable.vertex_array.unbind();
                renderable.index_buffer.unbind();


            }

            self.renderer.render_queue.pop_front();

            
        }
    }
}