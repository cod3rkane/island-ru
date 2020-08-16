use crate::core::game_state::GameState;
use glfw::{Action, Key, MouseButtonLeft, Window, WindowEvent};
use nalgebra_glm::{translate, vec2, vec3, Vec4, vec4, Mat4};

pub fn capture_input(window: &mut Window, event: &WindowEvent, game_state: &mut GameState) {
    match event {
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(0.0, -0.2, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(0.0, 0.2, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(0.2, 0.0, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
            let t = translate(&mut game_state.view_matrix, &vec3(-0.2, 0.0, 0.0));
            game_state.view_matrix = t;
        }
        glfw::WindowEvent::Scroll(x, y) => {
            if *y >= 1.0 {
                // scroll up
                if game_state.camera_zoom.abs() >= 8.0 {
                    let speed = 0.08;
                    let zoom = game_state.camera_zoom * -speed;
                    let t = translate(&mut game_state.view_matrix, &vec3(0.0, 0.0, zoom));
                    game_state.view_matrix = t;
                    game_state.camera_zoom = game_state.camera_zoom + zoom;
                }
            } else {
                // scroll down
                if game_state.camera_zoom.abs() <= 20.0 {
                    let speed = 0.08;
                    let zoom = game_state.camera_zoom * speed;
                    let t = translate(&mut game_state.view_matrix, &vec3(0.0, 0.0, zoom));
                    game_state.view_matrix = t;
                    game_state.camera_zoom = game_state.camera_zoom + zoom;
                }
            }
        }
        _ => {}
    }
}

pub fn capture_mouse(window: &mut Window, game_state: &mut GameState) {
    let (mouse_x, mouse_y) = window.get_cursor_pos();
    let x_offset: f32 = mouse_x as f32 - game_state.mouse_pos[0];
    let y_offset: f32 = mouse_y as f32 - game_state.mouse_pos[1];
    let mouse_left_state = window.get_mouse_button(MouseButtonLeft);
    let speed: f32 = 0.007;

    if mouse_left_state == Action::Press && (x_offset.abs() > 1.0 || y_offset.abs() > 1.0) {
        if mouse_x > game_state.mouse_pos[0].into() {
            let t = translate(&mut game_state.view_matrix, &vec3(x_offset.abs() * speed, 0.0, 0.0));
            game_state.view_matrix = t;
        }
        if mouse_x < game_state.mouse_pos[0].into() {
            let t = translate(&mut game_state.view_matrix, &vec3(x_offset.abs() * -speed, 0.0, 0.0));
            game_state.view_matrix = t;
        }
        if mouse_y < game_state.mouse_pos[1].into() {
            let t = translate(&mut game_state.view_matrix, &vec3(0.0, y_offset.abs() * speed, 0.0));
            game_state.view_matrix = t;
        }
        if mouse_y > game_state.mouse_pos[1].into() {
            let t = translate(&mut game_state.view_matrix, &vec3(0.0, y_offset.abs() * -speed, 0.0));
            game_state.view_matrix = t;
        }
    }

    game_state.mouse_pos = vec2(mouse_x as f32, mouse_y as f32);
}
