use std::{cell::{Cell, RefCell, RefMut}, collections::VecDeque, os::raw::c_void, rc::Rc};

use crate::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

use super::{renderable2d::Renderable2D, renderer::{Render, Renderer}, sprite, static_sprite::StaticSprite};
use gl::types::*;
pub struct Simple2dRenderer<'a> {
    pub renderer: Renderer<'a>,
}

impl<'a> Simple2dRenderer<'a> {
    pub fn new() -> Self {
        Self {
           renderer: Renderer::new(),
        }
    }
}



impl<'a> Render<'a> for Simple2dRenderer<'a> {


    fn submit(&mut self, sprite: &'a StaticSprite) {
        self.renderer.render_queue.push_back(sprite);
    }

    fn flush(&mut self) {
        while !self.renderer.render_queue.is_empty() {
            if let Some(sprite) = self.renderer.render_queue.front_mut() {


                sprite.vertex_array.bind();
                
                sprite.index_buffer.bind();

                sprite.shader.set_uniform_mat4("ml_matrix", Mat4::translation(&sprite.renderable2d.position));
                unsafe {
                    gl::DrawElements(gl::TRIANGLES, sprite.index_buffer.count as i32, gl::UNSIGNED_SHORT, 0  as *const c_void);

                }
                sprite.vertex_array.unbind();
                sprite.index_buffer.unbind();
            }
            self.renderer.render_queue.pop_front();
        }
    }
}