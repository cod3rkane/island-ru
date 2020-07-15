use gl::types::{ GLuint };

// pub enum LocationIndex {
//     VertexPosition = 0,
//     VertexColor = 1,
//     MatrixProjection = 2,
//     MatrixView = 3,
//     MatrixModel = 4,
// }

pub struct Shader {
    pub program_id: GLuint,
}
