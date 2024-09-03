use crate::index_buffer::IndexBuffer;
use crate::vertex_array::VertexArray;

use super::renderable2d::VertexData;
use super::renderer::{Render, Renderer};
use super::static_sprite::StaticSprite;
use gl::types::*;
use std::ptr::null;
use std::rc::Rc;
use std::cell::RefCell;

const MAX_RENDER_SPRITES: isize = 100_000;
const RENDER_VERTEX_SIZE: isize = size_of::<VertexData>() as isize;
const RENDER_SPRITE_SIZE: isize = RENDER_VERTEX_SIZE * 4;
const RENDER_BUFFER_SIZE: isize = RENDER_SPRITE_SIZE * MAX_RENDER_SPRITES;
const RENDERER_INDICES_SIZE: isize = MAX_RENDER_SPRITES * 6;

const SHADER_VERTEX_INDEX: u32 = 0;
const SHADER_COLOR_INDEX: u32 = 1;

struct BatchedRenderer2D<'a> {
    renderer: Renderer<'a>,
    vao: u32,
    ibo: IndexBuffer,
    index_cout: isize,
    vbo: u32,
}

impl BatchedRenderer2D<'_> {
    pub fn new() -> Self {
        let mut vbo = 0;
        let mut vao = 0;

        unsafe {
            gl::GenBuffers(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BufferData(gl::ARRAY_BUFFER, RENDER_BUFFER_SIZE, null(), gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(SHADER_VERTEX_INDEX);
            gl::EnableVertexAttribArray(SHADER_COLOR_INDEX);
            gl::VertexAttribPointer(SHADER_VERTEX_INDEX, 3, gl::FLOAT, gl::FALSE, RENDER_VERTEX_SIZE as i32, 0 as *const _);
            gl::VertexAttribPointer(SHADER_COLOR_INDEX, 4, gl::FLOAT, gl::FALSE, RENDER_VERTEX_SIZE as i32, (3 * size_of::<f32>()) as *const _);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }


        let mut indices: [u16; RENDERER_INDICES_SIZE as usize] = [0; RENDERER_INDICES_SIZE as usize];
        let mut offset = 0;

        for i in (0..RENDERER_INDICES_SIZE as usize).step_by(6) {
            indices[  i  ] = offset + 0;
            indices[i + 1] = offset + 1;
            indices[i + 2] = offset + 2;
    
            indices[i + 3] = offset + 2;
            indices[i + 4] = offset + 3;
            indices[i + 5] = offset + 0;
    
            offset += 4;
        }

        let ibo = IndexBuffer::from(&indices, RENDERER_INDICES_SIZE);
        unsafe {
            gl::BindVertexArray(0);
        }

        Self {
            renderer: Renderer::new(),
            vao,
            ibo,
            vbo,
            index_cout: 0,
        }
    }
}


impl<'a> Render<'a> for BatchedRenderer2D<'a> {
    fn submit(&mut self, renderable2d: &'a StaticSprite) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }
}


impl Drop for BatchedRenderer2D<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.vbo as *const _);
        }
    }
}