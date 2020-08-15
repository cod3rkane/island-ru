use gl::{
    CreateVertexArrays,
    BindVertexArray,
    DeleteVertexArrays,
    CreateBuffers,
    VertexAttribPointer,
    EnableVertexAttribArray,
    DeleteBuffers,
    BindBuffer,
};

use gl::types::{ GLuint, GLenum, GLvoid, GLsizeiptr, GLint, GLsizei, GLintptr };
extern crate gl;
use crate::core::{ texture_buffer::TextureBufferObject };

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BufferRenderType {
    DrawElements,
    DrawElementsInstanced,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Buffer {
    pub vao_id: GLuint,
    pub vertices_vbo: BufferObject,
    pub colors_vbo: BufferObject,
    pub indices_vbo: BufferObject,
    pub transformations_vbo: Option<BufferObject>,
    pub textures_vbo: Option<BufferObject>,
    pub textures_tbo: Option<TextureBufferObject>,
    pub render_type: BufferRenderType,
}

impl Buffer {
    pub fn new(render_type: BufferRenderType) -> Buffer {
        let mut vao_id: GLuint = 0;

        unsafe {
            CreateVertexArrays(1, &mut vao_id);
        }

        let transformations_vbo = match render_type {
            BufferRenderType::DrawElementsInstanced => Some(BufferObject::new(gl::ARRAY_BUFFER)),
            _ => None,
        };

        Buffer {
            vao_id,
            vertices_vbo: BufferObject::new(gl::ARRAY_BUFFER),
            colors_vbo: BufferObject::new(gl::ARRAY_BUFFER),
            indices_vbo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER),
            transformations_vbo,
            render_type,
            textures_vbo: Some(BufferObject::new(gl::ARRAY_BUFFER)),
            textures_tbo: Some(TextureBufferObject::new(gl::RG32F)),
        }
    }

    pub fn bind(&self) {
        unsafe {
            BindVertexArray(self.vao_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindVertexArray(0);
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

    pub fn unbind(&self) {
        unsafe {
            BindBuffer(self.kind, 0);
        }
    }

    pub fn set_data(&self, size: GLsizeiptr, data: *const GLvoid) {
        unsafe {
            gl::BufferData(
                self.kind,
                size,
                data,
                gl::DYNAMIC_DRAW
            );
        }
    }

    pub fn set_sub_data(&self, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid) {
        unsafe {
            gl::BufferSubData(
                self.kind,
                offset,
                size,
                data,
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

    pub fn set_vertex_attr_pointer(&self, index: GLuint, size: GLint, stride: GLsizei, pointer: *const GLvoid) {
        unsafe {
            VertexAttribPointer(
                index,
                size,
                gl::FLOAT,
                gl::FALSE,
                stride,
                pointer,
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

