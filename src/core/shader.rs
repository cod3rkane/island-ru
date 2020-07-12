use crate::components;

use components::shader::{ Shader };

use std::fs;

pub fn create_shader(vertexPath: &str, fragmentPath: &str) -> Shader {
    let vertexStr = fs::read_to_string(vertexPath).expect("Couldn't read the vertex path given.");
    let fragmentStr = fs::read_to_string(fragmentPath).expect("Couldn't read the fragment path given.");

    println!("vertex: {}", vertexStr);
    println!("fragment: {}", fragmentStr);

    Shader {
        id: 1,
        locs: None,
    }
}
