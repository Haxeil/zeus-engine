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
    let window = Window::from("zeus-rust".into(), 800, 600);

    let mut glfw = window.init();


    window.clear_color(1.0, 1.0, 1.0, 1.0);


  

    let vertices: [GLfloat; 9] = [
        0.0, 0.0, 0.0,
        15.0, 0.0, 0.0,
        0.0, 8.0, 0.0,
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

    unsafe  {
        let name = CString::new("pr_matrix").expect("can't");
        let location = gl::GetUniformLocation(shader.shader_id, name.as_ptr());
        println!("locationm : {location}");
        gl::UniformMatrix4fv(location, 1, gl::FALSE, ortho.elements.as_ptr());
        

    }

    while !window.closed() {
        window.clear();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }


        window.update(&mut glfw);
    }
}
