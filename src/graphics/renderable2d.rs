use glutin::dpi::Position;

use crate::{buffer::Buffer, index_buffer::IndexBuffer, vec2::Vec2, vec3::Vec3, vec4::Vec4, vertex_array::{self, VertexArray}};

use super::shader::Shader;

pub struct Renderable2D<'a> {

    pub size: Vec2,
    pub position: Vec3,
    pub color: Vec4,

    pub shader: &'a Shader,

    pub vertex_array: VertexArray<'a>,
    pub index_buffer: IndexBuffer,
}



impl<'a> Renderable2D<'a> {

    pub fn from(position: Vec3, size: Vec2, color: Vec4, shader: &'a Shader) -> Self {
        let mut vertex_array = VertexArray::new();

        let indicies: [u16; 6] = [0, 1, 2, 2, 3, 0];

        let verticies = [
            0.0, 0.0, 0.0,
            0.0, size.y, 0.0,
            size.x, size.y, 0.0,
            size.x, 0.0, 0.0,

        ];

        let colors = [
            color.x, color.y, color.z, color.w,
            color.x, color.y, color.z, color.w,
            color.x, color.y, color.z, color.w,
            color.x, color.y, color.z, color.w,

        ];

        vertex_array.add_buffer(&mut Buffer::from(&verticies, 12, 3), 0);
        vertex_array.add_buffer(&mut Buffer::from(&colors, 16, 4), 1);


        Self {
            position,
            size,
            color,

            vertex_array,
            index_buffer: IndexBuffer::from(&indicies, 6),
            shader,
            
        }
    }

}