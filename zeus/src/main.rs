mod core;
mod graphics;


use graphics::screen::Screen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use time::Time;
use crate::core::time;
use std::time::Instant;

fn main() -> Result<(), String> {
    let mut time = Time::default();
    let mut timer = Instant::now();
    let mut screen = Screen::new(1280, 720, String::from("Frimi d zab"))?;    
    
    while screen.running {
        time.update();
        
        while time.delta >= 1.0 {
            //update()
            time.updates += 1;
            time.delta -= 1.0;

        }

        time.frames += 1;
        // render 
        screen.render(Color::RGB(0, 0, 255))?;

        for event in screen.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    screen.running = false;
                    break;
                },
                _ => {}
            }
        }

        if timer.elapsed().as_millis() > 1_000 {
            timer = Instant::now();
            
            screen.canvas.window_mut().set_title(&format!(
                "Frimi d zab | {} up, {} fps, {} delta",
                time.updates, time.frames, time.delta
            )).unwrap();

            time.updates = 0;
            time.frames = 0;

        }


    }

    Ok(())

}
