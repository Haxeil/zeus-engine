use std::{cell::{Cell, RefCell}, collections::VecDeque, rc::Rc};

use super::renderable2d::Renderable2D;

pub struct Renderer<'a> {
    pub render_queue: VecDeque<Rc<RefCell<Renderable2D<'a>>>>,

}   

impl Renderer<'_> {

    pub fn new() -> Self {
        Self {
            render_queue: VecDeque::new()
        }
    }
}


pub trait Render<'a> {
    fn submit(&mut self, renderable2d: Rc<RefCell<Renderable2D<'a>>>);
    fn flush(&mut self);

}

