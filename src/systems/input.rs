use crate::core::game_state::{ GameState };
use glfw::{ Window, WindowEvent, Key, Action };
use nalgebra_glm::{ translate, vec3 };

pub fn capture_input(_window: &mut Window, event: &WindowEvent, game_state: &mut GameState) {
    match event {
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(0.0, -0.2, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(0.0, 0.2, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(0.2, 0.0, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(-0.2, 0.0, 0.0));
            game_state.view_matrix = t;
        }
        _ => {}
    }
}
