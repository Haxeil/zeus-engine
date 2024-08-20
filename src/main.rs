extern crate glfw;

mod graphics;
mod math;
use graphics::window::Window;
use math::vec2::Vec2;

fn main() {
    let window = Window::from("zeus-rust".into(), 800, 600);

    let mut glfw = window.init();

    window.clear_color(0.2, 1.0, 1.0, 1.0);

    let v1 = Vec2::zero();
    let mut v2 = Vec2::new(0.0, 2.0);

    v2 += v1;
    v2 *= v1;
    v2 -= v1;

    println!("res: {}", v2);

    while !window.closed() {
        window.clear();

        if window.is_key_pressed(glfw::Key::A) {
            println!("A");
        }

        window.update(&mut glfw);
    }
}
