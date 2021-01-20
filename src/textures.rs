extern crate gl;

use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use itertools::izip;
use thiserror::Error;

pub struct Texture {
    id: gl::types::GLuint,
    uniform: String,
}

pub struct TextureDescriptor<'a> {
    pub name: String,
    pub uniform: &'a str,
}

impl<'a> TextureDescriptor<'a> {
    pub fn new(name: &str, uniform: &'a str) -> Self {
        return Self {
            name: format!("{}/src/textures/{}", env!("CARGO_MANIFEST_DIR"), name),
            uniform,
        };
    }
}

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Could not load file: `{0}`")]
    Load(#[from] std::io::Error),
    #[error("Could not decode file: `{0}`")]
    Decode(#[from] image::ImageError),
}

impl Texture {
    pub fn from_files(
        image_descriptors: &[&TextureDescriptor],
    ) -> Result<Vec<Texture>, TextureError> {
        let images = load_from_files(image_descriptors)?;
        let textures = upload_to_gl(&images, image_descriptors);

        return Ok(textures);
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    pub fn uniform(&self) -> &String {
        return &self.uniform;
    }
}

fn upload_to_gl(images: &[DynamicImage], descriptor: &[&TextureDescriptor]) -> Vec<Texture> {
    let mut texture_id: gl::types::GLuint = 0;
    let mut result_ids: Vec<Texture> = Vec::new();

    for (_idx, image, &uniform_name) in izip!(0..images.len(), images, descriptor) {
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // How the texture should behave when it ends
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as gl::types::GLint,
            );

            // Visualization Filters.
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as gl::types::GLint,
            );

            // Load the texture in memory
            let (color_type, data_ptr) = match image.color() {
                image::ColorType::Rgb8 => (gl::RGB, image.as_rgb8().unwrap().as_ptr()),
                image::ColorType::Rgba8 => (gl::RGBA, image.as_rgba8().unwrap().as_ptr()),
                _ => panic!("Tipo nao tratado: {:?}", image.color()),
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,                 // Type of Texture
                0,                              // Minimap Level
                color_type as gl::types::GLint, // Type of image to be stored
                image.width() as gl::types::GLint,
                image.height() as gl::types::GLint,
                0,                 // Always zero. Legacy.
                color_type,        // Type of image to be read
                gl::UNSIGNED_BYTE, // Type of values we are passing.
                data_ptr as *const std::ffi::c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        result_ids.push(Texture {
            id: texture_id,
            uniform: String::from(uniform_name.uniform),
        });
    }

    return result_ids;
}

fn load_from_files(files: &[&TextureDescriptor]) -> Result<Vec<DynamicImage>, TextureError> {
    let mut images: Vec<DynamicImage> = Vec::new();

    // TODO: Move to another function `load_from_files`.
    for &image_file in files {
        let image_data = ImageReader::open(&image_file.name)?;
        let image_data = image_data.decode()?;
        let image_data = image_data.flipv();
        images.push(image_data);
    }

    return Ok(images);
}
