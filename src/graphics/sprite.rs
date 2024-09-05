use super::{renderable2d::{Renderable, Renderable2D}, shader::Shader};



pub struct Sprite {
    pub renderable2d: Renderable2D,

}


impl Sprite {
    pub fn from(renderable: Renderable2D) -> Self {
        Self {
            renderable2d: renderable
        }
    }
}

impl Renderable for Sprite {
    fn get_renderable(&self) -> &Renderable2D {
        
        &self.renderable2d
    }
}

