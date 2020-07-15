use gl::{
    CreateVertexArrays,
    BindVertexArray,
    DeleteVertexArrays,
    CreateBuffers,
    NamedBufferStorage,
    VertexAttribPointer,
    EnableVertexAttribArray,
    DeleteBuffers,
    BindBuffer,
};
use gl::types::{ GLuint, GLenum, GLvoid, GLsizeiptr, GLint, GLsizei };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Buffer {
    pub vao_id: GLuint,
    pub vertices_vbo: BufferObject,
    pub colors_vbo: BufferObject,
    pub indices_vbo: BufferObject,
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut vao_id: GLuint = 0;

        unsafe {
            CreateVertexArrays(1, &mut vao_id);
        }

        Buffer {
            vao_id,
            vertices_vbo: BufferObject::new(gl::ARRAY_BUFFER),
            colors_vbo: BufferObject::new(gl::ARRAY_BUFFER),
            indices_vbo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER),
        }
    }

    pub fn bind(&self) {
        unsafe {
            BindVertexArray(self.vao_id);
        }
    }

    pub fn clean(&self) {
        unsafe {
            DeleteVertexArrays(1, &self.vao_id);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferObject {
    pub id: GLuint,
    pub kind: GLenum,
}

impl BufferObject {
    pub fn new(kind: GLenum) -> BufferObject {
        let mut id: GLuint = 0;

        unsafe {
            CreateBuffers(1, &mut id);
        }

        BufferObject {
            id,
            kind,
        }
    }

    pub fn bind(&self) {
        unsafe {
            BindBuffer(self.kind, self.id);
        }
    }

    pub fn set_data(&self, size: GLsizeiptr, data: *const GLvoid) {
        unsafe {
            NamedBufferStorage(
                self.id,
                size,
                data,
                gl::DYNAMIC_STORAGE_BIT,
            );
        }
    }

    pub fn set_vertex_attr(&self, index: GLuint, size: GLint, stride: GLsizei) {
        unsafe {
            VertexAttribPointer(
                index,
                size,
                gl::FLOAT,
                gl::FALSE,
                stride,
                std::ptr::null(),
            );
            EnableVertexAttribArray(index);
        }
    }

    pub fn clean(&self) {
        unsafe {
            DeleteBuffers(1, &self.id);
        }
    }
}