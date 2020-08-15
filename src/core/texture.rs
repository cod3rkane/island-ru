use gl;
use gl::types::GLuint;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use std::os::raw::c_void;

#[derive(Clone)]
pub struct Texture {
    pub id: GLuint,
    pub image: Option<Box<DynamicImage>>,
    pub tile_width: i32,
    pub tile_height: i32,
    pub tile_coords_list: Vec<Vec<f32>>,
    pub tile_col_num: i32,
}

fn init_coord_list(col_num: i32, tile_size: f32) -> Vec<Vec<f32>> {
    let mut v: Vec<Vec<f32>> = vec![];

    for y in 0..col_num {
        for x in 0..col_num {
            let xo: f32 = x as f32 * tile_size;
            let yo: f32 = y as f32 * tile_size;
            v.push(vec![xo, yo]);
        }
    }

    v
}

fn get_tile_size(col_num: i32) -> f32 {
    let tile_size = 1.0 / col_num as f32;

    tile_size
}

fn get_col_num(image_size: (u32, u32), tile_height: i32) -> i32 {
    let (w, h) = image_size;
    let s = w as i32 / tile_height;

    s
}

impl Texture {
    pub fn new(image: Box<DynamicImage>, tile_size: i32) -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            let (width, height) = image.dimensions();
            let d_image: DynamicImage = image.flipv();
            let data = d_image.to_bytes();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        let image_col_num = get_col_num(image.dimensions(), tile_size);

        Texture {
            id,
            image: Some(image),
            tile_width: tile_size,
            tile_height: tile_size,
            tile_coords_list: init_coord_list(image_col_num, get_tile_size(image_col_num)),
            tile_col_num: image_col_num,
        }
    }

    /**
     ** return (start tile coord, end tile coord)
     */
    pub fn get_tile_coord(&self, index: usize) -> Vec<f32> {
        let pos = self.tile_coords_list.get(index).unwrap();
        let tile_size = get_tile_size(self.tile_col_num);
        let coords: Vec<f32> = vec![
            pos[0] + tile_size, pos[1] + tile_size,
            pos[0] + tile_size, pos[1],
            pos[0], pos[1],
            pos[0], pos[1] + tile_size,
        ];

        coords
    }

    /**
     ** return (start tile coord, end tile coord)
     */
    pub fn get_texture_coord_from_size(&self, index: usize, size: i32) -> Vec<f32> {
        let coord_list = self.get_list_coords_from_size(size);
        let pos = coord_list.get(index).unwrap();
        let image_col_num = get_col_num(self.image.as_ref().unwrap().dimensions(), size);
        let tile_size = get_tile_size(image_col_num);
        let coords: Vec<f32> = vec![
            pos[0] + tile_size, pos[1] + tile_size,
            pos[0] + tile_size, pos[1],
            pos[0], pos[1],
            pos[0], pos[1] + tile_size,
        ];

        coords
    }

    pub fn get_list_coords_from_size(&self, size: i32) -> Vec<Vec<f32>> {
        let image_col_num = get_col_num(self.image.as_ref().unwrap().dimensions(), size);

        init_coord_list(image_col_num, get_tile_size(image_col_num))
    }
}
