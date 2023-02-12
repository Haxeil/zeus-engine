use gl;
use std::mem::size_of;
use std::ops::{Add, Mul};

pub struct VertexBufferElement {
    pub v_type: GlType,
    pub count: u32,
    pub normalized: bool,
}

impl VertexBufferElement {
    pub fn new(v_type: GlType, count: u32, normalized: bool) -> Self {
        Self {
            v_type,
            count,
            normalized,
        }
    }

    pub fn size_of_current_type(&self) -> u32 {
        match self.v_type {
            GlType::GlUint(_) => size_of::<gl::types::GLuint>() as u32,
            GlType::GlInt(_) => size_of::<gl::types::GLint>() as u32,
            GlType::GlFloat(_) => size_of::<gl::types::GLfloat>() as u32,
            GlType::GlChar(_) => size_of::<gl::types::GLchar>() as u32,
        }
    }
}

pub struct VertexBufferLayout {
    pub elements: Vec<VertexBufferElement>,
    pub stride: u32,
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        Self {
            elements: vec![],
            stride: 0,
        }
    }

    pub fn push<T>(&mut self, count: u32)
    where
        T: Copy,
    {
        let s = size_of::<T>();
        if s == size_of::<i32>() {
            self.elements.push(VertexBufferElement::new(
                GlType::GlInt(0x1404),
                count,
                false,
            ));
        } else if s == size_of::<u32>() {
            self.elements.push(VertexBufferElement::new(
                GlType::GlUint(0x1405),
                count,
                false,
            ));
        } else if s == size_of::<f32>() {
            self.elements.push(VertexBufferElement::new(
                GlType::GlChar(0x1406),
                count,
                false,
            ));
        } else if s == size_of::<char>() {
            self.elements.push(VertexBufferElement::new(
                GlType::GlChar(0x1404),
                count,
                true,
            ));
        }

        self.stride += size_of::<T>() as u32 * count;
    }
}

pub enum GlType {
    GlUint(u32),
    GlInt(u32),
    GlFloat(u32),
    GlChar(u32),
}

impl GlType {
    pub fn get_value(&self) -> u32 {
        match *self {
            GlType::GlUint(value) => value,
            GlType::GlInt(value) => value,
            GlType::GlFloat(value) => value,
            GlType::GlChar(value) => value,
        }
    }
}
