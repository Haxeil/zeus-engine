extern crate glfw;

mod graphics;
mod math;
mod utils;
use rand::prelude::*;


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


    let mut renderer2d = BatchedRenderer2D::new();
    let mut simple_renderer = Simple2dRenderer::new();

    let renderable = Renderable2D::from(Vec3::new(0.0, 0.0, 0.0), Vec2::new(1.0, 1.0), Vec4::new(1.0, 1.0, 1.0, 1.0));

    let mut sprites: Vec<StaticSprite<'_>> = Vec::new();

    let mut rng = rand::thread_rng();

    shader.set_uniform_2f("light_pos", Vec2::new(4.0, 1.0)); // Use the temporary variable
    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));

    let mut y = 0.0;

    while y <= 9.0 {
        let mut x = 0.0;

        while x < 16.0 {

            sprites.push(StaticSprite::from(&shader, Renderable2D::from(
                Vec3::new(x, y, 0.0),
                Vec2::new(0.08, 0.08),
                Vec4::new(rng.gen_range(0..1000) as f32 / 1000.0, 1.0, 0.6, 1.0),
            )));
            x += 0.08;

        }

        y += 0.08;


    }

    println!("Sprites: {}", sprites.len());


    while !window.closed() {
        window.clear();

        let (x, y) = window.mouse_x_y;
        let pos = Vec2::new(x as f32 * 16.0 / 960.0, y as f32 * 9.0 / 540.0);

        shader.set_uniform_2f("light_pos", pos); // Use the temporary variable

        for sprite in sprites.iter() {
            simple_renderer.submit(sprite);
            simple_renderer.flush();
        }


        window.update(&mut glfw);
    }
}
