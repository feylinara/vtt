use std::mem::MaybeUninit;

use gl::{self, types::GLenum};
use gl::{types::GLuint, ReadBuffer};

use super::texture::Texture2D;

#[repr(transparent)]
pub struct FrameBuffer {
    pub(in crate::render) id: GLuint,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Attachment {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl Into<GLenum> for Attachment {
    fn into(self) -> GLenum {
        match self {
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

    pub fn attach_texture2D(texture: &Texture2D, attachment: Attachment) {
        texture.bind_current();
        unsafe { gl::FramebufferTexture(gl::FRAMEBUFFER, attachment.into(), gl::TEXTURE_2D, 0) }
    }

    pub fn attach_renderbuffer(renderbuffer: &RenderBuffer, attachment: Attachment) {
        renderbuffer.bind();
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                attachment.into(),
                gl::RENDERBUFFER,
                renderbuffer.id,
            )
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

pub struct RenderBuffer {
    pub(in crate::render) id: GLuint,
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
