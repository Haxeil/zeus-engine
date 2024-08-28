extern crate glfw;

mod graphics;
mod math;
mod utils;


use std::{mem, os::raw::c_void};

use gl::types::*;

use graphics::{shader::Shader, window::Window};
use mat4::Mat4;
use math::*;
use vec2::Vec2;
use vec3::Vec3;
use vec4::Vec4;



fn main() {
    let window = Window::from("zeus-rust".into(), 960, 540);

    let mut glfw = window.init();

    window.clear_color(1.0, 1.0, 1.0, 1.0);

    let vertices: [GLfloat; 18] = [
        0.0, 0.0, 0.0, 
        8.0, 0.0, 0.0, 
        0.0, 3.0, 0.0, 
        
        0.0, 3.0, 0.0, 
        8.0, 3.0, 0.0, 
        8.0, 0.0, 0.0,
    ];

    let mut vbo: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = (vertices.len() * mem::size_of::<f64>()) as isize;
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    let ortho = Mat4::orthographic(0.0, 16.0, 0.0, 9.0, -1.0, 1.0);

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    shader.set_uniform_mat4("pr_matrix", ortho);
    shader.set_uniform_mat4("ml_matrix", Mat4::translation(&Vec3::new(4.0, 3.0, 0.0)));
    
    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));
    shader.set_uniform_2f("light_pos", Vec2::new(4.0, 1.5));

    while !window.closed() {
        window.clear();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            let (x, y) = window.mouse_x_y;
            shader.set_uniform_2f("light_pos", Vec2::new(x as f32 * 16.0 / 960.0,  y as f32 * 9.0 / 540.0));
        }

        window.update(&mut glfw);
    }
}
