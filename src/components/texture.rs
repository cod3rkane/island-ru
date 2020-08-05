use gl::types::{ GLuint };

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    pub coordinates: Vec<f32>,
    pub textureID: GLuint,
}
