use std::cell::{Cell};

use glfw::{Context, Glfw, GlfwReceiver, Key, MouseButton, PWindow, WindowEvent};

const MAX_KEY_CODES: usize = 1024;
const MAX_BUTTONS: usize = 32;

// Global static mutable reference to Window
pub static mut GLOBAL_WINDOW: Option<Cell<Window>> = None;


pub fn get_global_window() -> &'static mut Window {
    unsafe { GLOBAL_WINDOW.as_mut().expect("Window not set").get_mut() }
}

#[derive(Debug)]
pub struct Window {
    pub name: String,
    pub width: i32,
    pub height: i32,
    closed: bool,
    pub window: Option<PWindow>,
    pub event: Option<GlfwReceiver<(f64, WindowEvent)>>,
    keys: [bool; MAX_KEY_CODES],
    mouse_buttons: [bool; MAX_BUTTONS],
    pub mouse_x_y: (f64, f64),
}

impl Default for Window {
    fn default() -> Self {
        Self {
            name: String::from(""),
            width: 0,
            height: 0,
            closed: false,
            window: Default::default(),
            event: Default::default(),
            keys: [false; MAX_KEY_CODES],
            mouse_buttons: [false; MAX_BUTTONS],
            mouse_x_y: (0.0, 0.0),
        }
    }
}

impl Window {
    
    pub fn from(name: String, width: i32, height: i32) -> &'static mut Self {
        let window = Window {
            name,
            width,
            height,
            ..Default::default()
        };

        unsafe {
            GLOBAL_WINDOW = Some(Cell::new(window));
        }

        get_global_window()

    }
}

impl Window {
    pub fn clear_color(&self, r: f32, g: f32, b: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(r, g, b, alpha);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn update(&mut self, glfw: &mut Glfw) {
        // Poll for and process events

        glfw.poll_events();

        // Swap front and back buffers
        let window = self.window.as_mut().expect("no window");

        (self.width, self.height) = window.get_framebuffer_size();

        window.swap_buffers();
    }

    pub fn closed(&mut self) -> bool {
        self.closed = self.window.as_ref().expect("no window").should_close();
        return self.closed;
    }

    pub fn init(&mut self) -> Glfw {
        let mut glfw: glfw::Glfw = glfw::init(error_callback).unwrap();

        (self.window, self.event) = {
            let (mut window, event) = glfw
                .create_window(
                    self.width as u32,
                    self.height as u32,
                    self.name.as_str(),
                    glfw::WindowMode::Windowed,
                )
                .expect("failed to Create window !");

            window.set_size_callback(window_resize);
            window.set_key_callback(key_callback);
            window.set_mouse_button_callback(mouse_callback);
            window.set_cursor_pos_callback(cursor_position_callback);

            (Some(window), Some(event))
        };

        // makes context current;
        glfw.make_context_current(self.window.as_deref());

        gl::load_with(|ptr| {
            self.window
                .as_mut()
                .expect("could not load GL")
                .get_proc_address(ptr) as *const _
        });
        unsafe {
            // make this into a function 
            //
            let version =
                std::str::from_utf8(std::slice::from_raw_parts(gl::GetString(gl::VENDOR), 7))
                    .expect("can't get string from gl");
            println!("GL Version: {}", version);
        }

        return glfw;
    }

    pub fn is_key_pressed(&mut self, keycode: Key) -> bool {
        let keycode = keycode as usize;
        // TODO: LOG
        if keycode >= MAX_KEY_CODES {
            return false;
        }
        self.keys[keycode]
    }

    pub fn is_mouse_button_pressed(&mut self, button: MouseButton) -> bool {
        let button = button as usize;
        // TODO: LOG
        if button >= MAX_BUTTONS {
            return false;
        }
        self.mouse_buttons[button]
    }
}

// CallBack Functions for glfw

fn error_callback(err: glfw::Error, description: String) {
    println!("GLFW error {:?}: {:?}", err, description);

    unsafe {
        glfw::ffi::glfwTerminate();
    }
}

fn window_resize(_window: &mut glfw::Window, x: i32, y: i32) {
    unsafe {
        gl::Viewport(0, 0, x, y);
    };
}

fn key_callback(
    _window: &mut glfw::Window,
    key: glfw::Key,
    _scan_code: glfw::Scancode,
    action: glfw::Action,
    _mods: glfw::Modifiers,
) {
    let global_window = get_global_window();

    global_window.keys[key as usize] = action != glfw::Action::Release;
}

// MouseButton, Action, Modifiers

fn mouse_callback(
    _window: &mut glfw::Window,
    mouse_action: glfw::MouseButton,
    action: glfw::Action,
    _mods: glfw::Modifiers,
) {
    let global_window = get_global_window();

    global_window.mouse_buttons[mouse_action as usize] = action != glfw::Action::Release;
}

fn cursor_position_callback(_window: &mut glfw::Window, x_pos: f64, y_pos: f64) {
    let global_window = get_global_window();

    global_window.mouse_x_y = (x_pos, y_pos);
}