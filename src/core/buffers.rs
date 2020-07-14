use gl::{
    CreateVertexArrays,
    BindVertexArray,
    DeleteVertexArrays,
    CreateBuffers,
    NamedBufferStorage,
    VertexAttribPointer,
    EnableVertexAttribArray,
};
use gl::types::{ GLuint, GLenum, GLvoid, GLsizeiptr, GLint, GLsizei };

pub struct VertexArrayObject {
    pub id: GLuint,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut id: GLuint = 0;

        unsafe {
            CreateVertexArrays(1, &mut id);
        }

        VertexArrayObject {
            id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            BindVertexArray(self.id);
        }
    }

    pub fn clean(&self) {
        unsafe {
            DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct BufferObject {
    pub id: GLuint,
    kind: GLenum,
    data: Option<*const GLvoid>,
    size: Option<GLsizeiptr>,
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
            data: None,
            size: None,
        }   
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind, self.id);
        }
    }

    pub fn set_data(&mut self, size: GLsizeiptr, data: *const GLvoid) {
        self.data = Some(data);
        self.size = Some(size);

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
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
