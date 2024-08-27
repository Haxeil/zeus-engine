extern crate glfw;

mod graphics;
mod math;
mod utils;
use std::{ffi::CString, mem, os::raw::c_void};

use gl::types::*;

use graphics::{window::Window, shader::Shader};
use mat4::Mat4;
use math::*;
use vec3::Vec3;
use vec4::Vec4;

fn main() {
    let window = Window::from("zeus-rust".into(), 960, 540);

    let mut glfw = window.init();


    window.clear_color(1.0, 1.0, 1.0, 1.0);


  

    let vertices: [GLfloat; 18] = [
        4.0, 3.0, 0.0,
        12.0, 3.0, 0.0,
        4.0, 6.0, 0.0,
        4.0, 6.0, 0.0,
        12.0, 6.0, 0.0,
        12.0, 3.0, 0.0,

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

    let ortho = Mat4::orthographic(0.0, 16.0, 0.0, 9.0, -1.0, 1.0);

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    shader.set_uniform_mat4("pr_matrix", ortho);
    shader.set_uniform_mat4("ml_matrix", Mat4::rotation(45.0, &Vec3::new(0.0, 0.0, 1.0)));
    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));

    while !window.closed() {
        window.clear();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }


        window.update(&mut glfw);
    }
}
