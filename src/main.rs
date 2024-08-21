extern crate glfw;

mod graphics;
mod math;

use graphics::window::Window;
use mat4::Mat4;
use math::*;

fn main() {
    let window = Window::from("zeus-rust".into(), 800, 600);

    let mut glfw = window.init();

    window.clear_color(0.2, 1.0, 1.0, 1.0);

    while !window.closed() {
        window.clear();

        if window.is_key_pressed(glfw::Key::A) {
            println!("A");
        }

        window.update(&mut glfw);
    }
}
