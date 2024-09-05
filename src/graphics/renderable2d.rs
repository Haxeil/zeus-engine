use glutin::dpi::Position;

use crate::{
    buffer::Buffer,
    index_buffer::IndexBuffer,
    vec2::Vec2,
    vec3::Vec3,
    vec4::Vec4,
    vertex_array::{self, VertexArray},
};

use super::shader::Shader;

#[derive(Debug)]
pub struct VertexData {
    pub vertex: Vec3,
    pub color: Vec4,
}

pub struct Renderable2D {
    pub size: Vec2,
    pub position: Vec3,
    pub color: Vec4,
}

impl Renderable2D {
    pub fn from(position: Vec3, size: Vec2, color: Vec4) -> Self {
        Self {
            position,
            size,
            color,
        }
    }
}

pub trait Renderable {
    fn get_renderable(&self) -> &Renderable2D;
}
