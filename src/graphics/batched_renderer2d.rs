use crate::index_buffer::IndexBuffer;
use crate::vec3::Vec3;
use crate::vertex_array::VertexArray;

use super::renderable2d::{Renderable, VertexData};
use super::renderer::{Render, Renderer};
use super::shader::Shader;
use super::sprite::Sprite;
use super::static_sprite::StaticSprite;
use gl::types::*;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::mem;
use std::os::raw::c_void;
use std::ptr::null;
use std::rc::Rc;

const MAX_RENDER_SPRITES: isize = 10_000;
const RENDER_VERTEX_SIZE: isize = size_of::<VertexData>() as isize;
const RENDER_SPRITE_SIZE: isize = RENDER_VERTEX_SIZE * 4;
const RENDER_BUFFER_SIZE: isize = RENDER_SPRITE_SIZE * MAX_RENDER_SPRITES;
const RENDERER_INDICES_SIZE: isize = MAX_RENDER_SPRITES * 6;

const SHADER_VERTEX_INDEX: u32 = 0;
const SHADER_COLOR_INDEX: u32 = 1;

pub struct BatchedRenderer2D {
    pub buffer: *mut c_void,
    vao: GLuint,
    ibo: IndexBuffer,
    index_count: GLint,
    vbo: GLuint,
}

impl BatchedRenderer2D {
    pub fn new() -> Self {
        let mut vbo = 0;
        let mut vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Correct stride: size of the entire VertexData structure (Vec3 + Vec4)

            gl::BufferData(
                gl::ARRAY_BUFFER,
                RENDER_BUFFER_SIZE,
                null(),
                gl::DYNAMIC_DRAW,
            );

            gl::EnableVertexAttribArray(SHADER_VERTEX_INDEX);
            gl::EnableVertexAttribArray(SHADER_COLOR_INDEX);

            gl::VertexAttribPointer(
                SHADER_VERTEX_INDEX,
                3,
                gl::FLOAT,
                gl::FALSE,
                RENDER_VERTEX_SIZE as i32,
                0 as *const _,
            );
            gl::VertexAttribPointer(
                SHADER_COLOR_INDEX,
                4,
                gl::FLOAT,
                gl::FALSE,
                RENDER_VERTEX_SIZE as i32,
                (3 * size_of::<GLfloat>()) as *const GLvoid,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        // Create indices
        let mut indices = Vec::<GLushort>::with_capacity(RENDERER_INDICES_SIZE as usize);
        let mut offset = 0;
        for _ in (0..RENDERER_INDICES_SIZE as usize).step_by(6) {
            indices.push(offset + 0);
            indices.push(offset + 1);
            indices.push(offset + 2);
            indices.push(offset + 2);
            indices.push(offset + 3);
            indices.push(offset + 0);
            offset += 4;
        }
        let ibo = IndexBuffer::from(&indices, RENDERER_INDICES_SIZE);

        unsafe {
            gl::BindVertexArray(0);
        }

        Self {
            buffer: 0 as *mut _,
            vao,
            ibo,
            vbo,
            index_count: 0,
        }
    }
}

impl BatchedRenderer2D {
    pub fn begin(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.buffer = gl::MapBuffer(gl::ARRAY_BUFFER, gl::READ_WRITE);
            // Ensure the buffer is successfully mapped
            if self.buffer.is_null() {
                println!("Failed to map buffer");
            }
        }
    }
    pub fn end(&mut self) {
        unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl<'a, T> Render<'a, T> for BatchedRenderer2D
where
    T: Renderable,
{
    fn submit(&mut self, sprite: &T) {
        let renderable = sprite.get_renderable();

        let position = renderable.position;
        let color = renderable.color;
        let size = renderable.size;

        unsafe {
            let vertex_data = self.buffer as *mut VertexData;
            let base_index = self.index_count;

            let vertex_base = vertex_data.add(base_index as usize);

            // First Vertex
            (*vertex_base).vertex = position;
            (*vertex_base).color = color;

            // Second Vertex
            (*vertex_base.add(1)).vertex = Vec3::new(position.x, position.y + size.y, position.z);
            (*vertex_base.add(1)).color = color;

            // Third Vertex
            (*vertex_base.add(2)).vertex =
                Vec3::new(position.x + size.x, position.y + size.y, position.z);
            (*vertex_base.add(2)).color = color;

            // Fourth Vertex
            (*vertex_base.add(3)).vertex = Vec3::new(position.x + size.x, position.y, position.z);
            (*vertex_base.add(3)).color = color;

            self.index_count += 6;
        }
    }

    fn flush(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao); // Bind VAO
            self.ibo.bind(); // Bind IndexBuffer

            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_SHORT,
                null(),
            );
            self.ibo.unbind(); // Unbind IndexBuffer
            gl::BindVertexArray(0); // Unbind VAO

            self.index_count = 0;
        }
    }
}
