use crate::components::{ entity::Entity, shader::Shader };
use crate::core::{ buffers::*, shader::create_shader, texture::Texture };

use nalgebra_glm::{ vec3, Mat4, mat4, translate, vec2, Vec2 };
extern crate image;
use image::GenericImage;
use image::GenericImageView;
use std::path::Path;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub buffers: Vec<Buffer>,
    pub current_shader: Shader,
    pub world_shader: Shader,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub window_width: i32,
    pub window_height: i32,
    pub viewport_width: i32,
    pub viewport_height: i32,
    pub world: Option<Entity>,
    pub textures: Option<Vec<Texture>>,
    pub mouse_pos: Vec2,
}

pub fn initial_game_state() -> GameState {
    let _initial_buffer: Buffer = Buffer::new(BufferRenderType::DrawElements);
    let _world_buffer: Buffer = Buffer::new(BufferRenderType::DrawElementsInstanced);
    let current_shader: Shader = create_shader("src/resources/vertex.glsl", "src/resources/fragment.glsl");
    let world_shader: Shader = create_shader("src/resources/vertex_world.glsl", "src/resources/fragment.glsl");
    let _texture_img = image::open(&Path::new("src/resources/textures/tiles-textures.png"))
        .expect("Failed to load texture!");
    let _texture: Texture = Texture::new(Box::new(_texture_img), 8);
    let mut _player = Entity::new_player(vec3(0.0, 0.0, 0.0), &_texture);
    _player.physics.as_mut().unwrap().scale(vec3(0.4, 0.4, 0.4));
    let mut _view_matrix: Mat4 = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    _view_matrix = translate(&mut _view_matrix, &vec3(0.0, 0.0, -8.0));
    let mut _projection_matrix: Mat4 = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let _texture_img = image::open(&Path::new("src/resources/textures/tiles-textures.png"))
        .expect("Failed to load texture!");
    let _texture: Texture = Texture::new(Box::new(_texture_img), 8);

    GameState {
        entities: vec![_player],
        buffers: vec![_world_buffer, _initial_buffer],
        current_shader,
        world_shader,
        view_matrix: _view_matrix,
        projection_matrix: _projection_matrix,
        window_width: 0,
        window_height: 0,
        viewport_width: 0,
        viewport_height: 0,
        world: Some(Entity::new_world(vec3(0.0, 0.0, 0.0), &_texture)),
        textures: Some(vec![_texture]),
        mouse_pos: vec2(0.0, 0.0),
    }
}
