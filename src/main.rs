extern crate glfw;

mod graphics;
mod math;
mod utils;


use std::{mem, os::raw::c_void};

use buffer::Buffer;
use gl::types::*;

use graphics::{shader::Shader, window::Window, buffers::*};
use index_buffer::IndexBuffer;
use mat4::Mat4;
use math::*;
use vec2::Vec2;
use vec3::Vec3;
use vec4::Vec4;
use vertex_array::VertexArray;



fn main() {
    let window = Window::from("zeus-rust".into(), 960, 540);

    let mut glfw = window.init();

    window.clear_color(1.0, 1.0, 1.0, 1.0);

    let vertices: [f32; 12] = [
        0.0, 0.0, 0.0, 
        0.0, 3.0, 0.0, 
        8.0, 3.0, 0.0, 
        8.0, 0.0, 0.0,
    ];

    let indicies: [GLushort; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    let colorsA = [
        1.0, 0.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0,

    ];


    let colorsB = [
        0.2, 0.3, 0.8, 1.0,
        0.2, 0.3, 0.8, 1.0,
        0.2, 0.3, 0.8, 1.0,
        0.2, 0.3, 0.8, 1.0,
    ];

    let mut sprite1 = VertexArray::new();
    let mut sprite2 = VertexArray::new();

    let mut vbo = Buffer::from(&vertices, 4 * 3, 3);
    let mut ibo = IndexBuffer::from(&indicies, 2 * 3);

    sprite1.add_buffer(&mut Buffer::from(&vertices, 4 * 3, 3), 0);
    sprite1.add_buffer(&mut Buffer::from(&colorsA, 4 * 4, 4), 1);

    sprite2.add_buffer(&mut  Buffer::from(&vertices, 4 * 3, 3), 0);
    sprite2.add_buffer(&mut  Buffer::from(&colorsB, 4 * 4, 4), 1);



    let ortho = Mat4::orthographic(0.0, 16.0, 0.0, 9.0, -1.0, 1.0);

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    shader.set_uniform_mat4("pr_matrix", ortho);
    
    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));
    shader.set_uniform_2f("light_pos", Vec2::new(4.0, 1.5));
    

    while !window.closed() {
        window.clear();

        unsafe {
    
            let (x, y) = window.mouse_x_y;
            shader.set_uniform_2f("light_pos", Vec2::new(x as f32 * 16.0 / 960.0,  y as f32 * 9.0 / 540.0));
            


            sprite1.bind();
            ibo.bind();
            shader.set_uniform_mat4("ml_matrix", Mat4::translation(&Vec3::new(4.0, 3.0, 0.0)));
            gl::DrawElements(gl::TRIANGLES, ibo.count as i32, gl::UNSIGNED_SHORT, std::ptr::null());
            sprite1.unbind();
            ibo.unbind();

            sprite2.bind();
            ibo.bind();
            shader.set_uniform_mat4("ml_matrix", Mat4::translation(&Vec3::new(0.0, 0.0, 0.0)));
            gl::DrawElements(gl::TRIANGLES, ibo.count as i32, gl::UNSIGNED_SHORT, std::ptr::null());
            sprite2.unbind();
            ibo.unbind();

        }

        window.update(&mut glfw);
    }
}
