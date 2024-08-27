extern crate glfw;

mod graphics;
mod math;

use gl::PointParameterf;
use graphics::window::Window;
use mat4::Mat4;
use math::*;
use vec3::Vec3;
use vec4::Vec4;

fn main() {
    let window = Window::from("zeus-rust".into(), 800, 600);

    let mut glfw = window.init();

    window.clear_color(0.2, 1.0, 1.0, 1.0);

    let a = Vec4::new(0.2, 0.3, 0.8, 1.0);
    let b = Vec4::new(0.5, 0.2, 0.1, 1.0);

    let c = a * b;

    let mut position = Mat4::translation(&Vec3::new(2.0, 3.0, 4.0));
    position *= Mat4::new_identity();

    position.column_mut(3).x = 2.0;

    while !window.closed() {
        window.clear();

        if window.is_key_pressed(glfw::Key::A) {
            println!("position elm: {:p}", &position.elements[12]);
            println!("position: {:p}", &position[3].x);
        }

        window.update(&mut glfw);
    }
}
