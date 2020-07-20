extern crate glfw;

use glfw::{ Action, Context, Key };

extern crate gl;

mod components;
mod core;
mod systems;

use crate::core::game_state::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(
        1280,
        720,
        "Island Something - Cod3r Kane",
        glfw::WindowMode::Windowed
    )
        .expect("Failed to Create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut _game_state: GameState = initial_game_state();

    while !window.should_close() {
        unsafe {
            gl::Viewport(0, 0, 1280, 720);

            gl::ClearColor(0.25098, 0.25098, 0.25098, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
        }

        for (_, e) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, e);
        }

        systems::physics::physics_system(&mut _game_state, glfw.get_time());
        systems::render::render_system(&mut _game_state);

        window.swap_buffers();
        glfw.poll_events();
    }

    systems::render::render_system_clean(&mut _game_state);
}

fn handle_window_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
