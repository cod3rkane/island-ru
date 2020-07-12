extern crate glfw;

use glfw::{ Action, Context, Key };

extern crate gl;

mod core;
mod components;

use components::mesh::{ Mesh };
use components::shader::{ Shader, LocationIndex };
use std::os::raw::c_void;

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

    let _primary_shader: Shader = core::shader::create_shader("src/resources/vertex.glsl", "src/resources/fragment.glsl");

    let _triangle: Mesh = Mesh {
        vertices: vec![
            -0.5, 0.0, 0.0,
            0.0, 0.5, 0.0,
            0.5, 0.0, 0.0,
            0.0, -0.5, 0.0,
        ],
        indices: vec![
            0, 1, 2,
            0, 2, 3,
        ],
        colors: vec![
            0.14902, 0.901961, 0.545098, 1.0,
            0.14902, 0.901961, 0.545098, 1.0,
            0.14902, 0.901961, 0.545098, 1.0,
            0.14902, 0.901961, 0.545098, 1.0,
        ],
    };

    let mut vao: gl::types::GLuint = 0;
    let mut vertices_vbo: gl::types::GLuint = 0;
    let mut colors_vbo: gl::types::GLuint = 0;
    let mut indices_vbo: gl::types::GLuint = 0;

    unsafe {
        gl::CreateVertexArrays(1, &mut vao);
        gl::CreateBuffers(1, &mut vertices_vbo);
        gl::CreateBuffers(1, &mut colors_vbo);
        gl::CreateBuffers(1, &mut indices_vbo);
    }

    unsafe {
        gl::BindVertexArray(vao);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vertices_vbo);
        gl::NamedBufferStorage(
            vertices_vbo,
            (_triangle.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            _triangle.vertices.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_STORAGE_BIT,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, colors_vbo);
        gl::NamedBufferStorage(
            colors_vbo,
            (_triangle.colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            _triangle.colors.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_STORAGE_BIT,
        );
        gl::VertexAttribPointer(
            1,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * std::mem::size_of::<f32>()) as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);
    }

    unsafe {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_vbo);
        gl::NamedBufferStorage(
            indices_vbo,
            (_triangle.indices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            _triangle.indices.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_STORAGE_BIT,
        );
    }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.25098, 0.25098, 0.25098, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::UseProgram(_primary_shader.program_id);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        for (_, e) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, e);
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vertices_vbo);
        gl::DeleteBuffers(1, &colors_vbo);
        gl::DeleteBuffers(1, &indices_vbo);
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
