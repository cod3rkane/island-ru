use gl::{ TextureBuffer, BindTextureUnit, TEXTURE_BUFFER, CreateTextures };
use gl::types::{ GLuint, GLenum };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureBufferObject {
    pub id: GLuint,
    pub format: GLenum,
}

impl TextureBufferObject {
    pub fn new(format: GLenum) -> TextureBufferObject {
        let mut id = 0;

        unsafe {
            CreateTextures(TEXTURE_BUFFER, 1, &mut id);
        }

        TextureBufferObject {
            id,
            format,
        }
    }

    pub fn attach_buffer(&self, buffer: GLuint) {
        unsafe {
            TextureBuffer(self.id, self.format, buffer);
        }
    }

    pub fn bind(&self, unit: GLuint) {
        unsafe {
            BindTextureUnit(unit, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindTextureUnit(0, 0);
        }
    }
}
