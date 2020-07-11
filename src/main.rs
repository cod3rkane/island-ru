extern crate glfw;

use glfw::{ Action, Context, Key };

fn main() {
    println!("Hello, world!");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(1280, 720, "Island Something - Cod3r Kane", glfw::WindowMode::Windowed).expect("Failed to Create GLFW Window");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();

        for (_, events) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, events);
        }
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
