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
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE); // Disable face culling
    }

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    shader.set_uniform_mat4("pr_matrix", ortho);
    println!("Projection matrix: {:?}", ortho);

    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));
    println!("Shader color uniform set: Vec4(0.2, 0.1, 0.3, 0.1)");

    let mut renderer2d = BatchedRenderer2D::new();

    let renderable = Renderable2D::from(
        Vec3::new(5.0, 5.0, 0.0),
        Vec2::new(4.0, 4.0),
        Vec4::new(1.0, 0.4, 0.45, 1.0),
    );
    let sprite_1 = Sprite::from(renderable);
    let renderable = Renderable2D::from(
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec4::new(0.1, 0.4, 0.45, 1.0),
    );
    let sprite_2 = Sprite::from(renderable);

    let mut simple_renderer = Simple2dRenderer::new();
    let renderable = Renderable2D::from(
        Vec3::new(1.0, 1.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec4::new(0.1, 0.4, 0.45, 1.0),
    );
    let sprite_3 = StaticSprite::from(&shader, renderable);

    while !window.closed() {
        window.clear();

        let (x, y) = window.mouse_x_y;
        let pos = Vec2::new(x as f32 * 16.0 / 960.0, y as f32 * 9.0 / 540.0);
        shader.set_uniform_2f("light_pos", pos); // Use the temporary variable

        renderer2d.begin();

        renderer2d.submit(&sprite_1); // Immutable borrow
        renderer2d.submit(&sprite_2); // Immutable borrow
        renderer2d.end();

        <BatchedRenderer2D as Render<'_, Sprite>>::flush(&mut renderer2d);

        simple_renderer.submit(&sprite_3);
        simple_renderer.flush();

        window.update(&mut glfw);
    }
}
