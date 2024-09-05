use crate::index_buffer::IndexBuffer;
use crate::vec3::Vec3;
use crate::vertex_array::VertexArray;

use super::renderable2d::{Renderable, VertexData};
use super::renderer::{Render, Renderer};
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
            let stride = std::mem::size_of::<VertexData>() as GLsizei;

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
                stride,
                0 as *const _,
            );
            gl::VertexAttribPointer(
                SHADER_COLOR_INDEX,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * size_of::<GLfloat>()) as *const GLvoid,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        // Create indices
        let mut indices = Vec::<u16>::with_capacity(RENDERER_INDICES_SIZE as usize);
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

            self.buffer = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY);

            // Ensure the buffer is successfully mapped
            if self.buffer.is_null() {
                println!("Failed to map buffer");
            }
        }
    }
    pub fn end(&mut self) {
        unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
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

        // Log data to ensure it's being populated correctly
        println!(
            "Submitting sprite with position: {:?}, size: {:?}, color: {:?}",
            position, size, color
        );
        unsafe {
            let vertex_data = self.buffer as *mut VertexData;
            let base_index = self.index_count;

            // Ensure there's enough space in the buffer
            let vertex_size = std::mem::size_of::<VertexData>();
            let required_size = (base_index + 4) * vertex_size as i32;

            // Compute base index
            let vertex_base = vertex_data.add(base_index as usize);

            // First Vertex
            (*vertex_base).vertex = position;
            (*vertex_base).color = color;
            println!(
                "Vertex 1: Position: {:?}, Color: {:?}",
                (*vertex_base).vertex,
                (*vertex_base).color
            );

            // Second Vertex
            (*vertex_base.add(1)).vertex = Vec3::new(position.x, position.y + size.y, position.z);
            (*vertex_base.add(1)).color = color;
            println!(
                "Vertex 2: Position: {:?}, Color: {:?}",
                (*vertex_base.add(1)).vertex,
                (*vertex_base.add(1)).color
            );

            // Third Vertex
            (*vertex_base.add(2)).vertex =
                Vec3::new(position.x + size.x, position.y + size.y, position.z);
            (*vertex_base.add(2)).color = color;
            println!(
                "Vertex 3: Position: {:?}, Color: {:?}",
                (*vertex_base.add(2)).vertex,
                (*vertex_base.add(2)).color
            );

            // Fourth Vertex
            (*vertex_base.add(3)).vertex = Vec3::new(position.x + size.x, position.y, position.z);
            (*vertex_base.add(3)).color = color;
            println!(
                "Vertex 4: Position: {:?}, Color: {:?}",
                (*vertex_base.add(3)).vertex,
                (*vertex_base.add(3)).color
            );

            self.index_count += 6;
        }
    }

    fn flush(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao); // Bind VAO
            self.ibo.bind(); // Bind IndexBuffer

            // Check if VAO is bound
            let mut current_vao: GLint = 0;
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
            println!("Bound VAO: {}", current_vao);

            // Draw the elements
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

// impl Drop for BatchedRenderer2D<'_> {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteBuffers(1, self.vbo as *const _);
//             gl::BindBuffer(gl::ARRAY_BUFFER, 0);

//         }
//     }
// }
