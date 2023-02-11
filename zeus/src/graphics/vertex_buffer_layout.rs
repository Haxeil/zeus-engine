use crate::graphics::renderer::*;
use gl;
use std::mem::size_of;
use std::ops::{Add, Mul};
use std::process::Output;

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
            GlType::gl_uint(_) => size_of::<gl::types::GLuint>() as u32,
            GlType::gl_int(_) => size_of::<gl::types::GLint>() as u32,
            GlType::gl_float(_) => size_of::<gl::types::GLfloat>() as u32,
            GlType::gl_char(_) => size_of::<gl::types::GLchar>() as u32,
            _ => todo!(),
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
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        let s = size_of::<T>();
        if s == size_of::<i32>() {
            self.elements.push(VertexBufferElement::new(
                GlType::gl_int(0x1404),
                count,
                false,
            ));
        } else if s == size_of::<u32>() {
            self.elements.push(VertexBufferElement::new(
                GlType::gl_uint(0x1405),
                count,
                false,
            ));
        } else if s == size_of::<f32>() {
            self.elements.push(VertexBufferElement::new(
                GlType::gl_float(0x1406),
                count,
                false,
            ));
        } else if s == size_of::<char>() {
            self.elements.push(VertexBufferElement::new(
                GlType::gl_char(0x1404),
                count,
                true,
            ));
        }

        self.stride += size_of::<T>() as u32 * count;
    }
}

pub enum GlType {
    gl_uint(u32),
    gl_int(u32),
    gl_float(u32),
    gl_char(u32),
}

impl GlType {
    pub fn get_value(&self) -> u32 {
        match *self {
            GlType::gl_uint(value) => value,
            GlType::gl_int(value) => value,
            GlType::gl_float(value) => value,
            GlType::gl_char(value) => value,
        }
    }
}
