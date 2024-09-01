use std::{cell::{Cell, RefCell, RefMut}, collections::VecDeque, os::raw::c_void, rc::Rc};

use crate::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

use super::{renderable2d::Renderable2D, renderer::{Renderer}};
use gl::types::*;
pub struct Simple2dRenderer<'a> {
    pub render_queue: VecDeque<Rc<RefCell<Renderable2D<'a>>>>,
}

impl Simple2dRenderer<'_> {
    pub fn new() -> Self {
        Self {
           render_queue: VecDeque::new(),
        }
    }
}



impl<'a> Simple2dRenderer<'a> {


    pub fn submit(&mut self, renderable2d: Rc<RefCell<Renderable2D<'a>>>) {
        self.render_queue.push_back(renderable2d);
    }

    pub fn change_pos(&mut self, position: Vec3) {
        // self.render_queue.front_mut().unwrap().position = position;
    }

    pub fn flush(&mut self) {
        while !self.render_queue.is_empty() {
            // WHY DO I HAVE A DOUBLE REFERENCE !!!!!!!!!!!!!!!!!!!
            if let Some(renderable) = self.render_queue.front_mut() {

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

            self.render_queue.pop_front();

            
        }
    }
}