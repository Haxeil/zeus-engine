extern crate glfw;

mod graphics;
mod math;
mod utils;



use std::{cell::RefCell, rc::Rc};



use graphics::{buffers::*, renderable2d::Renderable2D, shader::Shader, simple2d_renderer::Simple2dRenderer, window::Window};
use mat4::Mat4;
use math::*;
use vec2::Vec2;
use vec3::Vec3;
use vec4::Vec4;




fn main() {
    let window = Window::from("zeus-rust".into(), 960, 540);

    let mut glfw = window.init();

    window.clear_color(1.0, 1.0, 1.0, 1.0);


    let ortho = Mat4::orthographic(0.0, 16.0, 0.0, 9.0, -1.0, 1.0);

    let mut shader = Shader::from("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    shader.enable();

    shader.set_uniform_mat4("pr_matrix", ortho);
    
    shader.set_uniform_4f("colour", Vec4::new(0.2, 0.1, 0.3, 0.1));
    shader.set_uniform_2f("light_pos", Vec2::new(4.0, 1.5));

    let mut renderer2d = Simple2dRenderer::new();

    let mut sprite = Rc::new(RefCell::new(Renderable2D::from(Vec3::new(5.0, 0.0, 0.0), Vec2::new(2.0, 4.0), Vec4::new(1.0, 0.4, 0.45, 1.0), &shader)));
    
    while !window.closed() {
        window.clear();
    
        let (x, y) = window.mouse_x_y;
        let pos = Vec2::new(x as f32 * 16.0 / 960.0, y as f32 * 9.0 / 540.0);
    
        shader.set_uniform_2f("light_pos", pos); // Use the temporary variable

        // Now mutate the original position
         // Mutable borrow
        
        let size = sprite.borrow().size;
        sprite.borrow_mut().position = Vec3::new(pos.x - size.x / 2.0 , pos.y - size.y / 2.0 , 0.0);


        renderer2d.submit(sprite.clone()); // Immutable borrow

        renderer2d.flush();


        window.update(&mut glfw);
    }

}
