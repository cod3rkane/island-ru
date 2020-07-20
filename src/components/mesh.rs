#[derive(Debug, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<i32>,
    pub colors: Vec<f32>,
}
