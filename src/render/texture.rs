use gl;
use gl::types::GLuint;
use image::{DynamicImage, GenericImageView};
use std::os::raw::c_void;

trait Texture {
    fn get_id(&self) -> GLuint;
}

fn delete_texture<T: Texture>(texture: &mut T) {
    unsafe {
        gl::DeleteTextures(1, &mut texture.get_id() as *mut _);
    }
}

#[derive(Eq, PartialEq)]
pub struct Texture1D {
    pub(in crate::render) id: GLuint,
}

#[derive(Eq, PartialEq)]
pub struct Texture2D {
    pub(in crate::render) id: GLuint,
}

#[derive(Eq, PartialEq)]
pub struct Texture3D {
    pub(in crate::render) id: GLuint,
}

impl Texture for Texture2D {
    fn get_id(&self) -> GLuint {
        self.id
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Rgb,
    Rgba,
    Bgr,
    Bgra,
}

impl Format {
    fn into_internal_format(self) -> i32 {
        (match self {
            Self::Rgb => gl::RGB8,
            Self::Rgba => gl::RGBA8,
            Self::Bgr => gl::RGB8,
            Self::Bgra => gl::RGBA8,
        }) as i32
    }
    fn into_format(self) -> u32 {
        match self {
            Self::Rgb => gl::RGB,
            Self::Rgba => gl::RGBA,
            Self::Bgr => gl::BGR,
            Self::Bgra => gl::BGRA,
        }
    }
}

fn prepare_image(image: DynamicImage) -> DynamicImage {
    match image {
        DynamicImage::ImageRgb8(_)
        | DynamicImage::ImageRgba8(_)
        | DynamicImage::ImageBgr8(_)
        | DynamicImage::ImageBgra8(_) => image,
        DynamicImage::ImageLuma8(_)
        | DynamicImage::ImageLuma16(_)
        | DynamicImage::ImageRgb16(_) => DynamicImage::ImageRgb8(image.into_rgb8()),
        DynamicImage::ImageLumaA8(_)
        | DynamicImage::ImageLumaA16(_)
        | DynamicImage::ImageRgba16(_) => DynamicImage::ImageRgba8(image.into_rgba8()),
    }
}

fn data_ptr(image: &DynamicImage) -> *const c_void {
    match &image {
        DynamicImage::ImageRgb8(img) => img.as_ptr() as *const c_void,
        DynamicImage::ImageRgba8(img) => img.as_ptr() as *const c_void,
        DynamicImage::ImageBgr8(img) => img.as_ptr() as *const c_void,
        DynamicImage::ImageBgra8(img) => img.as_ptr() as *const c_void,
        _ => panic!("render::texture::data_ptr called with non-converted image"),
    }
}
fn format(image: &DynamicImage) -> Format {
    match &image {
        DynamicImage::ImageRgb8(_) => Format::Rgb,
        DynamicImage::ImageRgba8(_) => Format::Rgba,
        DynamicImage::ImageBgr8(_) => Format::Bgr,
        DynamicImage::ImageBgra8(_) => Format::Bgra,
        _ => panic!("render::texture::format called with non-converted image"),
    }
}

impl Texture2D {
    /// Create a Texture2D from an Image.
    ///
    /// We take the image by value so we can always convert it into a
    /// format OpenGL will support. If you know you don't need this
    /// use one of the direct construction functions
    /// TODO (write those functions)
    pub fn from_image(image: DynamicImage) -> Texture2D {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id as *mut _);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }
        let image = prepare_image(image);
        let data = data_ptr(&image);
        let format = format(&image);
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format.into_internal_format(),
                image.width() as i32,
                image.height() as i32,
                0,
                format.into_format(),
                gl::UNSIGNED_BYTE,
                data as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Texture2D { id }
    }

    pub fn with_dimensions(width: i32, height: i32, format: Format) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id as *mut _);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format.into_internal_format(),
                width as i32,
                height as i32,
                0,
                format.into_format(),
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Texture2D { id }
    }

    pub fn replace_rect(&mut self, x: i32, y: i32, image: DynamicImage) {
        self.bind(0);
        let image = prepare_image(image);
        let data = data_ptr(&image);
        let format = format(&image);
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                x,
                y,
                image.width() as i32,
                image.height() as i32,
                format.into_format(),
                gl::UNSIGNED_BYTE,
                data as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn bind(&self, idx: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + idx);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn bind_current(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
