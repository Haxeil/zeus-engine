extern crate alloc;

mod core;
mod graphics;
use crate::core::time;
use crate::graphics::renderer::*;

use crate::graphics::index_buffer::IndexBuffer;
use crate::graphics::shader::*;
use crate::graphics::vertex_array::VertexArray;
use crate::graphics::vertex_buffer::VertexBuffer;
use crate::graphics::vertex_buffer_layout::VertexBufferLayout;
use gl::types::GLchar;
use glfw::{Action, Context, Key};
use std::ffi::c_void;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
use std::{
    alloc::{alloc, Layout},
    error::Error,
    ffi::{c_char, CStr, CString},
    mem::size_of,
    ptr::null,
    time::Instant,
};
use time::Time;

fn main() -> Result<(), Box<dyn Error>> {
    let mut time = Time::default();
    let mut timer = Instant::now();
    // TODO: abstract all of GL related stuff in Screen struct;
    //let event_loop = EventLoop::new();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    #[cfg(target_os = "macos")]
    println!("I'm apple machine");

    #[cfg(target_os = "macos")]
    unsafe {
        glfw::ffi::glfwWindowHint(glfw::ffi::OPENGL_FORWARD_COMPAT, 1);
    }

    let (mut window, events) = glfw.with_connected_monitors(|glfw, m| {
        let _monitor = m.first().unwrap();
        glfw.create_window(1280, 720, "Zeb", glfw::WindowMode::Windowed)
            .expect("can't create window")
    });

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    // Make the window's context current

    window.make_current();

    let version = unsafe { CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_str()? };

    println!("{:?}", version);

    let positions: [f32; 4 * 2] = [-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];

    let indicies: [u32; 3 * 2] = [0, 1, 2, 2, 3, 0];

    let vertex_array = VertexArray::new().construct();

    let vertex_buffer = VertexBuffer::new().construct(
        positions.as_ptr() as *const c_void,
        12 as isize * size_of::<f32>() as isize,
    );

    let mut layout = VertexBufferLayout::new();
    layout.push::<i32>(2);
    vertex_array.add_buffer(&vertex_buffer, &layout);

    let index_buffer = IndexBuffer::new().construct(indicies.as_ptr() as *const _, 6);

    let shader = Shader::new("src/res/shaders/Basic.shader").construct()?;
    shader.bind();
    shader.set_uniform_4f("u_Color", 0.3, 0.1, 0.0, 1.0);

    window.set_key_polling(true);
    // insuring that the the window won't stuck at the machine refresh rate;

    glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

    index_buffer.unbind();
    vertex_buffer.unbind();
    vertex_array.unbind();
    shader.unbind();

    let mut i: f32 = 0.0;
    let mut increament = 0.05_f32;

    while !window.should_close() {
        time.update();
        time.frames += 1;
        while time.delta >= 1.0 {
            //update()
            time.updates += 1;
            time.delta -= 1.0;
            log_gl_error!(gl::Clear(gl::COLOR_BUFFER_BIT));

            shader.bind();
            vertex_array.bind();
            index_buffer.bind();

            shader.set_uniform_4f("u_Color", 1.0 / i, i, 0.0, 1.0);
            log_gl_error!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null()));

            i += increament * time.delta as f32;

            if i >= 1.0 {
                increament = -0.05;
            } else if i < 0.0 {
                increament = 0.05;
            }

            log_gl_error!(gl::ClearColor(0.12, 0.12, 0.13, 1.0));
        }

        time.frames += 1;

        // Poll for and process events
        glfw.poll_events();
        window.swap_buffers();

        if timer.elapsed().as_millis() > 1_000 {
            timer = Instant::now();

            window.set_title(&format!(
                "Zeus | {} up, {} fps, {} delta, i: {}",
                time.updates, time.frames, time.delta, i
            ));

            time.updates = 0;
            time.frames = 0;
        }

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    Ok(())
}

// TODO: Abstract all of this into Screen struct
