use gl::{ TexBuffer, BindTextureUnit, BindTexture, TEXTURE_BUFFER, /*CreateTextures*/ GenTextures, TEXTURE_2D };
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
            GenTextures(1, &mut id);
        }

        TextureBufferObject {
            id,
            format,
        }
    }

    pub fn attach_buffer(&self, buffer: GLuint) {
        unsafe {
            TexBuffer(TEXTURE_BUFFER, self.format, buffer);
        }
    }

    pub fn bind(&self, unit: GLuint) {
        unsafe {
            BindTexture(TEXTURE_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindTexture(TEXTURE_BUFFER, 0);
        }
    }
}
