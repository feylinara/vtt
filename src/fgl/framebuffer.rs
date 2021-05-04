use std::mem::MaybeUninit;

use gl::types::GLuint;
use gl::{self, types::GLenum};

use super::texture::Texture2D;

#[repr(transparent)]
pub struct FrameBuffer {
    pub(in crate::fgl) id: GLuint,
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id as *const _);
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Attachment {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl From<Attachment> for GLenum {
    fn from(attachment: Attachment) -> GLenum {
        match attachment {
            Attachment::Color(x) => gl::COLOR_ATTACHMENT0 + x,
            Attachment::Depth => gl::DEPTH_ATTACHMENT,
            Attachment::Stencil => gl::STENCIL_ATTACHMENT,
            Attachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

impl FrameBuffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id as *mut GLuint);
        }
        Self { id }
    }

    pub fn new_array<const N: usize>() -> [Self; N] {
        let mut buffers: [Self; N] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            gl::GenFramebuffers(N as i32, &mut buffers[0] as *mut Self as *mut GLuint);
        }
        buffers
    }

    pub fn attach_texture2d(&self, texture: &Texture2D, attachment: Attachment) {
        self.bind();
        unsafe { gl::FramebufferTexture(gl::FRAMEBUFFER, attachment.into(), texture.id, 0) }
        self.unbind();
    }

    pub fn attach_renderbuffer(&self, renderbuffer: &RenderBuffer, attachment: Attachment) {
        self.bind();
        renderbuffer.bind();
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                attachment.into(),
                gl::RENDERBUFFER,
                renderbuffer.id,
            )
        }
        self.unbind();
    }

    pub fn clear_color<C: ColorClearable>(&self, buffer: i32, color: &[C]) {
        self.bind();
        unsafe { C::clear(buffer, color.as_ptr()) }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

pub trait ColorClearable {
    unsafe fn clear(drawbuffer: i32, value: *const Self);
}

impl ColorClearable for i32 {
    unsafe fn clear(drawbuffer: i32, value: *const Self) {
        gl::ClearBufferiv(gl::COLOR, drawbuffer, value);
    }
}

impl ColorClearable for u32 {
    unsafe fn clear(drawbuffer: i32, value: *const Self) {
        gl::ClearBufferuiv(gl::COLOR, drawbuffer, value);
    }
}

impl ColorClearable for f32 {
    unsafe fn clear(drawbuffer: i32, value: *const Self) {
        gl::ClearBufferfv(gl::COLOR, drawbuffer, value);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Rgba,
    Depth,
    DepthStencil,
}

impl Format {
    fn into_internal_format(self) -> u32 {
        (match self {
            Self::Rgba => gl::RGBA8,
            Self::Depth => gl::DEPTH_COMPONENT16,
            Self::DepthStencil => gl::DEPTH24_STENCIL8,
        })
    }
}

pub struct RenderBuffer {
    pub(in crate::fgl) id: GLuint,
}

impl RenderBuffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut id as *mut GLuint);
        }
        Self { id }
    }

    pub fn new_array<const N: usize>() -> [Self; N] {
        let mut buffers: [Self; N] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            gl::GenRenderbuffers(N as i32, &mut buffers[0] as *mut Self as *mut GLuint);
        }
        buffers
    }

    pub fn alloc(&self, width: u32, height: u32, format: Format, samples: u32) {
        self.bind();
        unsafe {
            gl::RenderbufferStorageMultisample(
                gl::RENDERBUFFER,
                samples as i32,
                format.into_internal_format(),
                width as i32,
                height as i32,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.id as *const _);
        }
    }
}
