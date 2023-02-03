mod core;
mod graphics;
use crate::core::time;
use glfw::{ffi::glfwGetWindowMonitor, Action, Context, Key, Window};
use std::{
    
    ffi::{c_char, CStr, CString},
    mem::size_of,
    ptr::{null, null_mut},
    time::Instant, alloc::{alloc, Layout},
};
use time::Time;

fn main() -> Result<(), String> {
    env_logger::init();
    let mut time = Time::default();
    let mut timer = Instant::now();
    // TODO: abstract all of GL related stuff in Screen struct;
    //let event_loop = EventLoop::new();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.with_connected_monitors(|glfw, m| {
        let monitor = m.first().unwrap();
        glfw.create_window(1280, 720, "title", glfw::WindowMode::Windowed)
            .expect("can't get window")
    });

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    // Make the window's context current
    window.make_current();
    unsafe {
        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
            .to_str()
            .unwrap();

        println!("{:?}", version);

        let mut buffer: u32 = 1;
        let positions: [f32; 6] = [-0.5, -0.5, 0.0, 0.5, 0.5, -0.5];

        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            6 * size_of::<&f32>() as isize,
            positions.as_ptr() as *const _,
            gl::STATIC_DRAW,
        )
    }
    window.set_key_polling(true);
    // insuring that the the window won't stuck at the machine refrech rate;

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    

    glfw.set_swap_interval(glfw::SwapInterval::None);
    let mut i = 0_f32;




    // Loop until the user closes the window
//     glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
// glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
// glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
// glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    unsafe {
        // gl::ClearColor(i, 0.3, 0.3 / i, 1.0);
        // gl::Clear(gl::COLOR_BUFFER_BIT);


        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 8, 0 as *const _);

        let vertex_shader = String::from(
            "
                #version 330 core\n
                \n
                layout(location = 0) in vec4 position;
                \n
                void main()\n
                {\n
                    gl_Position = position;\n
                }\n
                ",
        );
        let fragment_shader = String::from(
            "
                #version 330 core\n
                \n
                layout(location = 0) out vec4 color;
                \n
                void main()\n
                {\n
                    color = vec4(1.0, 1.0, 0.5, 1.0);\n
                }\n
                ",
        );

        let shader: u32 = create_shader(vertex_shader, fragment_shader);
        gl::UseProgram(shader);
    }
    while !window.should_close() {
        time.update();
        time.frames += 1;
        while time.delta >= 1.0 {
            //update()
            time.updates += 1;
            time.delta -= 1.0;
            i = (i + 0.01) % 1.0;
            unsafe {
                gl::ClearColor(i, 0.3, 0.3 / i, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                gl::EnableVertexAttribArray(0);
            }
        }

        time.frames += 1;

        // Poll for and process events
        glfw.poll_events();
        window.swap_buffers();

        if timer.elapsed().as_millis() > 1_000 {
            timer = Instant::now();

            window.set_title(&format!(
                "Frimi d zab | {} up, {} fps, {} delta",
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

unsafe fn create_shader(vertex_shader: String, fragment_shader: String) -> u32 {
    let program = gl::CreateProgram();
    let vs = compile_shader(gl::VERTEX_SHADER, vertex_shader);
    let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_shader);
    // Attach Shaders;
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    gl::ValidateProgram(program);
    // Drop Shaders;
    gl::DeleteShader(vs);
    gl::DeleteShader(vs);

    program
}

unsafe fn compile_shader(c_type: u32, source: String) -> u32 {
    let id = gl::CreateShader(c_type);
    // let src = source.as_bytes().as_ptr() as *const *const i8;
    let c_string = CString::new(source).unwrap();
    let source = c_string.as_ptr();
    let source = &source as *const *const i8;

    gl::ShaderSource(id, 1, source, std::ptr::null());
    gl::CompileShader(id);

    let mut result = 0;
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut result as *mut i32);

    if result as u8 == gl::FALSE {
        let mut length: i32 = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length as *mut _);
        let layout = std::alloc::Layout::from_size_align(length.try_into().unwrap(), 1).unwrap();
        let message: *mut c_char = std::alloc::alloc(layout) as *mut i8;
        gl::GetShaderInfoLog(id, length, &mut length as *mut _, message);
        println!(
            "Failed to compile: {}",
            CStr::from_ptr(message).to_str().unwrap()
        );

        gl::DeleteShader(id);
        return 0;
    }
    id
}


// TODO: Abstract all of this into Screen struct