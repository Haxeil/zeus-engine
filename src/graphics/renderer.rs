use std::{cell::{Cell, RefCell}, collections::VecDeque};

use super::renderable2d::Renderable2D;

pub struct Renderer<'a> {
    pub render_queue: VecDeque<Renderable2D<'a>>,

}   

impl Renderer<'_> {

    pub fn new() -> Self {
        Self {
            render_queue: VecDeque::new()
        }
    }
}


// pub trait Render<'a> {
//     fn submit(&mut self, renderable2d: &'a Renderable2D<'a>);
//     fn flush(&mut self);

// }

