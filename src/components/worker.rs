
#[derive(Clone)]
pub struct Worker {
    pub texture_coords: Vec<f32>,
}

impl Worker {
    pub fn new(texture_coords: Vec<f32>) -> Self {
        Worker {
            texture_coords,
        }
    }
}
