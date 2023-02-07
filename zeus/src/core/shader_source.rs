pub struct ShaderSource {
    pub vertex_shader: String,
    pub fragment_shader: String,
}

impl ShaderSource {
    pub fn new() -> Self {
        Self {
            vertex_shader: String::new(),
            fragment_shader: String::new(),
        }
    }
}