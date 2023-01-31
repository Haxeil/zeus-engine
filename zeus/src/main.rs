mod core;
mod graphics;


use graphics::screen::Screen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use time::Time;
use crate::core::time;
use std::time::Instant;

fn main() {
    let mut time = Time::default();
    let mut timer = Instant::now();
    let mut screen = Screen::new(400, 600, String::from("Frimi d zab"));
    // maybe it should be handled in the Screen struct ?
    let mut event_pump = screen.sdl_context.event_pump()
        .expect("can't get event pump ");
    

    let mut i = 0;
    
    while screen.running {
        time.update();
        
        while time.delta >= 1.0 {
            //update()
            time.updates += 1;
            time.delta -= 1.0;
            i = (i + 1) % 255;

        }

        time.frames += 1;
        // render 
        screen.canvas.set_draw_color(Color::RGB(i, 0, 255));
        screen.canvas.clear();
        screen.canvas.present();

        for event in event_pump.poll_iter() {
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
            )).expect("can't set tittle");
            time.updates = 0;
            time.frames = 0;
        }

    }


}
