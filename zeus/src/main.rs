extern crate alloc;

mod core;
mod graphics;
use crate::core::time;
use crate::graphics::renderer::*;

use crate::core::shader_source::ShaderSource;
use crate::graphics::index_buffer::IndexBuffer;
use crate::graphics::vertex_buffer::VertexBuffer;
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
    env_logger::init();
    let mut time = Time::default();
    let mut timer = Instant::now();
    // TODO: abstract all of GL related stuff in Screen struct;
    //let event_loop = EventLoop::new();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

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
            .expect("can't get window")
    });

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    // Make the window's context current
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    unsafe {
        log_gl_error!(gl::GenVertexArrays(1, &mut vao));
        log_gl_error!(gl::GenBuffers(1, &mut vbo));
        // Bind the Vertex Array Object first, then bind and set vertex buffer(s) and attribute pointer(s).
        log_gl_error!(gl::BindVertexArray(vao));
    }

    window.make_current();

    unsafe {
        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
            .to_str()
            .unwrap();

        println!("{:?}", version);
    }

    let positions: [f32; 4 * 2] = [-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];

    let indicies: [u32; 3 * 2] = [0, 1, 2, 2, 3, 0];

    let mut shader = 0;
    let mut vertex_buffer = VertexBuffer::new().construct(
        positions.as_ptr() as *const c_void,
        12 as isize * size_of::<f32>() as isize,
    );

    log_gl_error!(gl::VertexAttribPointer(
        0,
        2,
        gl::FLOAT,
        gl::FALSE,
        8,
        0 as *const _
    ));

    let mut index_buffer = IndexBuffer::new().construct(indicies.as_ptr() as *const _, 6);

    let shaders = unsafe { parse_shader("src/res/shaders/Basic.shader")? };
    shader = unsafe { create_shader(shaders.vertex_shader, shaders.fragment_shader)? };
    log_gl_error!(gl::UseProgram(shader));

    window.set_key_polling(true);
    // insuring that the the window won't stuck at the machine refresh rate;

    glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

    log_gl_error!(let location = unsafe {gl::GetUniformLocation(shader, "u_Color".as_ptr() as *const GLchar)});
    assert_ne!(location, -1);
    let mut i: f32 = 0.0;
    let mut increament = 0.05_f32;
    while !window.should_close() {
        time.update();
        time.frames += 1;
        while time.delta >= 1.0 {
            //update()
            time.updates += 1;
            time.delta -= 1.0;
            unsafe {
                log_gl_error!(gl::Clear(gl::COLOR_BUFFER_BIT));
                log_gl_error!(gl::Uniform4f(location, i, 0.5 / i, 0.1, 1.0));
                log_gl_error!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null()));

                index_buffer.bind();
                i += increament;

                if i > 1.0 {
                    increament = -0.05;
                } else if i < 1.0 {
                    increament = 0.05;
                } else {
                    i = 0.0;
                }

                log_gl_error!(gl::ClearColor(0.12, 0.12, 0.13, 1.0));
                log_gl_error!(gl::EnableVertexAttribArray(0));
            }
        }

        time.frames += 1;

        // Poll for and process events
        glfw.poll_events();
        window.swap_buffers();

        if timer.elapsed().as_millis() > 1_000 {
            timer = Instant::now();

            window.set_title(&format!(
                "Zeus | {} up, {} fps, {} delta",
                time.updates, time.frames, time.delta
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

unsafe fn create_shader(
    vertex_shader: String,
    fragment_shader: String,
) -> Result<u32, Box<dyn Error>> {
    let program = gl::CreateProgram();
    let vs = compile_shader(gl::VERTEX_SHADER, vertex_shader)?;
    let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_shader)?;
    // Attach Shaders;
    log_gl_error!(gl::AttachShader(program, vs));
    log_gl_error!(gl::AttachShader(program, fs));
    log_gl_error!(gl::LinkProgram(program));
    log_gl_error!(gl::ValidateProgram(program));
    // Drop Shaders;
    log_gl_error!(gl::DeleteShader(vs));

    Ok(program)
}

unsafe fn compile_shader(c_type: u32, source: String) -> Result<u32, Box<dyn Error>> {
    let id = gl::CreateShader(c_type);
    // let src = source.as_bytes().as_ptr() as *const *const i8;
    let c_string = alloc::ffi::CString::new(source)?;
    let source = c_string.as_ptr();
    let source = &source as *const *const i8;

    log_gl_error!(gl::ShaderSource(id, 1, source, null()));
    log_gl_error!(gl::CompileShader(id));

    let mut result = 0;
    log_gl_error!(gl::GetShaderiv(
        id,
        gl::COMPILE_STATUS,
        &mut result as *mut i32
    ));

    if result as u8 == gl::FALSE {
        let mut length: i32 = 0;
        log_gl_error!(gl::GetShaderiv(
            id,
            gl::INFO_LOG_LENGTH,
            &mut length as *mut _
        ));
        let layout = Layout::from_size_align(length.try_into()?, 1)?;
        let message: *mut c_char = alloc(layout) as *mut i8;
        log_gl_error!(gl::GetShaderInfoLog(
            id,
            length,
            &mut length as *mut _,
            message
        ));
        println!("Failed to compile: {}", CStr::from_ptr(message).to_str()?);

        log_gl_error!(gl::DeleteProgram(id));
        return Ok(0);
    }
    Ok(id)
}

unsafe fn parse_shader(path: &str) -> Result<ShaderSource, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut shaders = ShaderSource::new();
    let reader = BufReader::new(file);
    let mut is_fragment_shader = false;

    for line in reader.lines() {
        let line = line?;
        if line.ends_with("vertex") {
            continue;
        }

        if line.ends_with("fragment") {
            is_fragment_shader = true;
            continue;
        }

        if is_fragment_shader {
            shaders.fragment_shader.push_str(line.trim());
            shaders.fragment_shader.push_str("\n");
        } else {
            shaders.vertex_shader.push_str(line.trim());
            shaders.vertex_shader.push_str("\n");
        }
    }

    Ok(shaders)
}

// TODO: Abstract all of this into Screen struct
