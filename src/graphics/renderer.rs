use std::{cell::{Cell, RefCell}, collections::VecDeque, rc::Rc};

use super::{renderable2d::Renderable2D, static_sprite::StaticSprite};

pub struct Renderer<'a> {
    pub render_queue: VecDeque<&'a StaticSprite<'a>>,

}   

impl Renderer<'_> {

    pub fn new() -> Self {
        Self {
            render_queue: VecDeque::new()
        }
    }
}


pub trait Render<'a> {
    fn submit(&mut self, renderable2d: &'a StaticSprite);
    fn flush(&mut self);

}

