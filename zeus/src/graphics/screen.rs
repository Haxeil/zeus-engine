use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{Sdl};
// Struct for handling window related stuff
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub running: bool,
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Screen {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        let sdl_context = sdl2::init().expect("can't get sdl context");
        let video_subsystem = sdl_context.video().expect("can't get video ");

        let window = video_subsystem.window(&title, width, height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        
        let canvas = window.into_canvas().build().expect("Can't get canvas");
        
        Self {
            width,
            height,
            title,
            running: true,
            sdl_context,
            canvas,
        }
    }


}



