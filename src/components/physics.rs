use nalgebra_glm::{ Vec3, vec1, Mat4, mat4, translate, rotate_z, scale, radians };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Physics {
    pub position: Vec3,
    pub transform: Mat4,
}

impl Physics {
    pub fn new(position: Vec3) -> Physics {
        let mut _transform: Mat4 = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
        );

        _transform = translate(&mut _transform, &position);

        Physics {
            position,
            transform: _transform,
        }
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let radians = radians(&vec1(angle));
        self.transform = rotate_z(&mut self.transform, radians.x);
    }

    pub fn translate(&mut self, position: Vec3) {
        self.transform = translate(&mut self.transform, &position);
    }

    pub fn scale(&mut self, s: Vec3) {
        self.transform = scale(&mut self.transform, &s);
    }
}
