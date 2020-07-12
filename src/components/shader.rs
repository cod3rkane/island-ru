pub enum LocationIndex {
    VertexPosition = 0,
    VertexColor = 1,
    MatrixProjection,
    MatrixView,
    MatrixModel,
}

pub struct Shader {
    pub id: u8,
    pub locs: Option<Vec<u8>>,
}
