extern crate gl;

use image::{GenericImageView, DynamicImage};
use image::io::Reader as ImageReader;

pub struct Texture {
    ids: Vec<gl::types::GLuint>,
}

pub enum TextureError {
    Load(String),
    Decode(String),
}

impl Texture {
    pub fn new(image_files : &[&str]) -> Result<Texture, TextureError> {

        let images = load_from_files(image_files)?;
        let texture_ids = upload_to_gl(&images);
        return Ok(Texture{ids: texture_ids});
    }

    pub fn ids(&self) -> &Vec<gl::types::GLuint> {
        return &self.ids;
    }
}

fn upload_to_gl(images: &Vec<DynamicImage>) -> Vec<gl::types::GLuint> {
    let mut texture_id: gl::types::GLuint = 0;
    let mut result_ids: Vec<gl::types::GLuint> = Vec::new();

    for image_data in images {
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // How the texture should behave when it ends
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);

            // Visualization Filters.
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);

            // Load the texture in memory
            // TODO: Change the gl::RGB below to be Image Aware.
            gl::TexImage2D(
                gl::TEXTURE_2D,     // Type of Texture
                0,                  // Minimap Level
                gl::RGB as gl::types::GLint,            // Type of image to be stored
                image_data.width() as gl::types::GLint,
                image_data.height() as gl::types::GLint,
                0,                  // Always zero. Legacy.
                gl::RGB,            // Type of image to be read
                gl::UNSIGNED_BYTE,  // Type of values we are passing.
                image_data.as_rgb8().expect("Error converting").as_ptr() as *const std::ffi::c_void
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        result_ids.push(texture_id);
    }
    return result_ids;
}

fn load_from_files(files: &[&str]) -> Result<Vec<DynamicImage>, TextureError> {
    let mut images: Vec<DynamicImage> = Vec::new();

    // TODO: Move to another function `load_from_files`.
    for image_file in files {
        let image_data = ImageReader::open(image_file);
        let image_data = match image_data {
            Ok(image_data) => image_data,
            Err(e) => return Err(TextureError::Load(e.to_string())),
        };

        let image_data = image_data.decode();
        let image_data = match image_data {
            Ok(data)=> data,
            Err(e) => return Err(TextureError::Decode(e.to_string())),
        };
        images.push(image_data);
    }
    return Ok(images);
}
