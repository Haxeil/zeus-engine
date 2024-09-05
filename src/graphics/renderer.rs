use std::{cell::{Cell, RefCell}, collections::VecDeque, rc::Rc};

use super::{renderable2d::{Renderable, Renderable2D}, sprite::Sprite, static_sprite::StaticSprite};

pub struct Renderer<'a, T> {
    pub render_queue: VecDeque<&'a T>,

}   

impl<T> Renderer<'_, T> {

    pub fn new() -> Self {
        Self {
            render_queue: VecDeque::new()
        }
    }
}


pub trait Render<'a, T> 
    where T: Renderable
{
    fn submit(&mut self, renderable2d: &'a T);
    fn flush(&mut self) where T: Renderable;


}

