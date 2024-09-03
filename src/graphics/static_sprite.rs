use crate::{buffer::{self, Buffer}, index_buffer::IndexBuffer, vertex_array::VertexArray};

use super::{renderable2d::Renderable2D, renderer::Render, shader::Shader};



pub struct StaticSprite<'a> {

    pub renderable2d: &'a Renderable2D,
    
    pub shader: &'a Shader,
    pub vertex_array: VertexArray<'a>,
    pub index_buffer: IndexBuffer,
}

impl<'a> StaticSprite<'a> {

    pub fn from(shader: &'a Shader, renderable2d: &'a Renderable2D) -> Self {
        let mut vertex_array = VertexArray::new();

        let indicies: [u16; 6] = [0, 1, 2, 2, 3, 0];

        let size = renderable2d.size;

        let verticies = [
            0.0, 0.0, 0.0,
            0.0, size.y, 0.0,
            size.x, size.y, 0.0,
            size.x, 0.0, 0.0,

        ];

        let color = renderable2d.color;

        let colors = [
            color.x, color.y, color.z, color.w,
            color.x, color.y, color.z, color.w,
            color.x, color.y, color.z, color.w,
            color.x, color.y, color.z, color.w,

        ];

        let mut buffer = Buffer::from(&verticies, 12, 3);
        vertex_array.add_buffer(&mut buffer, 0);
        
        let mut buffer = Buffer::from(&colors, 16, 4);
        vertex_array.add_buffer(&mut buffer, 1);

        Self {
            renderable2d,
            vertex_array,
            shader,
            index_buffer: IndexBuffer::from(&indicies, 6),
        }
    }
}