use image::{ DynamicImage };
use gl::types::{ GLuint };

#[derive(Clone)]
pub struct Texture {
    pub coordinates: Vec<f32>,
    pub image: Option<Box<DynamicImage>>,
}
