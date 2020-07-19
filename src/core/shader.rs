use crate::components::shader::{ Shader };

use std::fs;
use gl::types::{ GLuint, GLenum, GLchar, GLint };
use std::ffi::{CString, CStr};

pub fn compile_shader(source: &CStr, kind: GLenum) -> GLuint {
    let shader: GLuint;

    unsafe {
        shader = gl::CreateShader(kind);
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    // @TODO: check if it was compiled successfully

    shader
}

pub fn create_program(vertex: GLuint, fragment: GLuint) -> GLuint {
    let program: GLuint;

    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);

        gl::BindAttribLocation(program, 0, CString::new("vertexPosition").expect("vertexPosition").as_ptr());
        gl::BindAttribLocation(program, 1, CString::new("vertexColor").expect("vertexColor").as_ptr());

        gl::LinkProgram(program);
    }

    let mut success: GLint = 1;

    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;

        unsafe {
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar
            );
        }

        println!("result: {}", error.to_string_lossy().into_owned());
    }

    program
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn create_shader(vertex_path: &str, fragment_path: &str) -> Shader {
    let vertex_str = fs::read_to_string(vertex_path).expect("Couldn't read the vertex path given.");
    let fragment_str = fs::read_to_string(fragment_path).expect("Couldn't read the &fragment path given.");
    let vertex_cstr = CString::new(vertex_str).unwrap();
    let fragment_cstr = CString::new(fragment_str).unwrap();

    let vertex_id: GLuint = compile_shader(&vertex_cstr, gl::VERTEX_SHADER);
    let fragment_id: GLuint = compile_shader(&fragment_cstr, gl::FRAGMENT_SHADER);

    let program_id: GLuint = create_program(vertex_id, fragment_id);

    Shader {
        program_id,
    }
}
