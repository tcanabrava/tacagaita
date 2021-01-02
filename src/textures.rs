extern crate gl;

use image::{GenericImageView};
use image::io::Reader as ImageReader;

pub struct Texture {
    id: gl::types::GLuint,
}

pub enum TextureError {
    Load(String),
    Decode(String),
}

impl Texture {
    pub fn new(image_file : &str) -> Result<Texture, TextureError> {

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


        //-------- Texture -----------
        // Jesus, that's complex.
        let mut texture_id = 0;
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
        return Ok(Texture{id: texture_id});
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }
}