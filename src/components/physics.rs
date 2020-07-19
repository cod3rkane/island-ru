use nalgebra_glm::{ Vec3, Mat4, mat4, translate, rotate_z };

pub struct Physics {
    pub position: Vec3,
    pub transform: Mat4,
}

impl Physics {
    pub fn new(position: Vec3) -> Physics {
        let mut matrix: Mat4 = mat4(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        matrix = translate(&mut matrix, &position);

        Physics {
            position,
            transform: matrix,
        }
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.transform = rotate_z(&mut self.transform, angle);
    }
}
