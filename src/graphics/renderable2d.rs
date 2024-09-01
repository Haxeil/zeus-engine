use glutin::dpi::Position;

use crate::{buffer::Buffer, index_buffer::IndexBuffer, vec2::Vec2, vec3::Vec3, vec4::Vec4, vertex_array::{self, VertexArray}};

pub struct Renderable2D<'a> {

    pub size: Vec2,
    pub position: Vec3,
    pub color: Vec4,

    vertex_array: VertexArray<'a>,
    index_buffer: IndexBuffer,
}

impl<'a> Renderable2D<'a> {

    pub fn from(position: Vec3, size: Vec2, color: Vec4) -> Self {

        let indicies: [u16; 6] = [0, 1, 2, 2, 3, 0];
        Self {
            position,
            size,
            color,

            vertex_array: VertexArray::new(),
            index_buffer: IndexBuffer::from(&indicies, 6),
            
        }
    }

}