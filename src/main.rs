extern crate glfw;

use glfw::{ Action, Context, Key };

extern crate gl;

mod core;
mod components;

use components::mesh::{ Mesh };
use components::shader::{ Shader };

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(1280, 720, "Island Something - Cod3r Kane", glfw::WindowMode::Windowed)
        .expect("Failed to Create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let primary_shader = core::shader::create_shader("src/resources/vertex.glsl", "src/resources/fragment.glsl");

    let _triangle: Mesh = Mesh {
        vertices: vec![
            0.0, 0.5, 0.0,
            -0.5, 0.0, 0.0,
            0.5, 0.0, 0.0,
        ],
        indices: Some(vec![
            0, 1, 2,
            1, 2, 3,
        ]),
        colors: vec![
            0.878431, 0.384314, 0.301961,
            0.878431, 0.384314, 0.301961,
            0.878431, 0.384314, 0.301961,
        ],
    };

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.25098, 0.25098, 0.25098, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for (_, e) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, e);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
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
