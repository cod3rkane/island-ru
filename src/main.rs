extern crate glfw;

use glfw::{ Action, Context, Key };

fn main() {
    println!("Hello, world!");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(1280, 720, "Island Something - Cod3r Kane", glfw::WindowMode::Windowed)
        .expect("Failed to Create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    while !window.should_close() {
        for (_, events) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, events);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
