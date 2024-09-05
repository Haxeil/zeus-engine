extern crate glfw;

mod graphics;
mod math;
mod utils;

use graphics::{
    batched_renderer2d::BatchedRenderer2D, buffers::*, renderable2d::Renderable2D,
    renderer::Render, shader::Shader, simple2d_renderer::Simple2dRenderer, sprite::Sprite,
    static_sprite::StaticSprite, window::Window,
};
use mat4::Mat4;
use math::*;
use vec2::Vec2;
use vec3::Vec3;
use vec4::Vec4;

fn main() {
    let window = Window::from("zeus-rust".into(), 960, 540);

    let mut glfw = window.init();

    window.clear_color(0.0, 0.0, 0.0, 1.0);

    let ortho = Mat4::orthographic(0.0, 16.0, 0.0, 9.0, -1.0, 1.0);

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    shader.set_uniform_mat4("pr_matrix", ortho);

    let renderable = Renderable2D::from(
        Vec3::new(-1003.0, 2.0, 0.0),
        Vec2::new(100_000_000.0, 7.0),
        Vec4::new(0.2, 0.0, 1.0, 1.0),
    );

    let mut renderer2d = BatchedRenderer2D::new();
    let sprite_1 = Sprite::from(renderable);

    shader.set_uniform_2f("light_pos", Vec2::new(4.0, 1.0)); // Use the temporary variable
    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));

    while !window.closed() {
        window.clear();

        let (x, y) = window.mouse_x_y;
        let pos = Vec2::new(x as f32 * 16.0 / 960.0, y as f32 * 9.0 / 540.0);

        shader.set_uniform_2f("light_pos", pos); // Use the temporary variable

        renderer2d.begin();

        renderer2d.submit(&sprite_1); // Immutable borrow
        renderer2d.end();

        <BatchedRenderer2D as Render<'_, Sprite>>::flush(&mut renderer2d);

        window.update(&mut glfw);
    }
}
