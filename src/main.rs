extern crate glfw;

mod graphics;
mod math;
mod utils;

use graphics::{window::Window, shader::Shader};
use mat4::Mat4;
use math::*;
use vec3::Vec3;
use vec4::Vec4;

fn main() {
    let window = Window::from("zeus-rust".into(), 800, 600);

    let mut glfw = window.init();

    let shader = Shader::from("src/shaders/vertex.shader", "src/shaders/fragment.shader");

    window.clear_color(0.2, 1.0, 1.0, 1.0);



    while !window.closed() {
        window.clear();

        if window.is_key_pressed(glfw::Key::A) {
        }

        window.update(&mut glfw);
    }
}
