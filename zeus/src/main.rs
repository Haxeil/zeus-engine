mod core;
mod graphics;

use crate::core::time;
use glfw::{ffi::glfwGetWindowMonitor, Action, Context, Key, Window};
use std::time::Instant;
use time::Time;

fn main() -> Result<(), String> {
    let mut time = Time::default();
    let mut timer = Instant::now();
    // TODO: abstract all of GL related stuff in Screen struct;
    //let event_loop = EventLoop::new();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.with_connected_monitors(|glfw, m| {
        let monitor = m.first().unwrap();
        glfw.create_window(1280, 720, "title", glfw::WindowMode::FullScreen(monitor))
            .expect("can't get window")
    });
    
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    // Make the window's context current
    window.make_current();
 
    window.set_key_polling(true);
    // insuring that the the window won't stuck at the machine refrech rate;
    glfw.set_swap_interval(glfw::SwapInterval::None);
    let mut i = 0_f32;
    // Loop until the user closes the window
    while !window.should_close() {
        time.update();
        time.frames += 1;
        while time.delta >= 1.0 {
            //update()
            time.updates += 1;
            time.delta -= 1.0;
            i = (i + 0.01) % 1.0;
        }

        time.frames += 1;
        unsafe {
            gl::ClearColor(i, 0.3, 0.3 / i, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
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

// let window = WindowBuilder::new().build(&event_loop).unwrap();

//     event_loop.run(move |event, _, control_flow| match event {
//         Event::WindowEvent {
//             ref event,
//             window_id,
//         } if window_id == window.id() => match event {
//             WindowEvent::CloseRequested
//             | WindowEvent::KeyboardInput {
//                 input:
//                     KeyboardInput {
//                         state: ElementState::Pressed,
//                         virtual_keycode: Some(VirtualKeyCode::Escape),
//                         ..
//                     },
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             _ => {}
//         },
//         _ => {}
//     });
