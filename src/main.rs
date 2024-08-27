extern crate glfw;

mod graphics;
mod math;
mod utils;
use std::{mem, os::raw::c_void};

use gl::types::*;

use graphics::{window::Window, shader::Shader};
use mat4::Mat4;
use math::*;
use vec3::Vec3;
use vec4::Vec4;

fn main() {
    let window = Window::from("zeus-rust".into(), 800, 600);

    let mut glfw = window.init();


    window.clear_color(1.0, 1.0, 1.0, 1.0);


  

    let vertices: [GLfloat; 18] = [
        -0.5, -0.5, 0.0,
        -0.5,  0.5, 0.0,
         0.5,  0.5, 0.0,
         0.5,  0.5, 0.0,
         0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
    ];

    let mut vbo: GLuint = 0;

    unsafe { 
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = (vertices.len() * mem::size_of::<f64>()) as isize;
        gl::BufferData(gl::ARRAY_BUFFER,  size, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    while !window.closed() {
        window.clear();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }


        window.update(&mut glfw);
    }
}
