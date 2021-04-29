use gl;
use gl::types::GLuint;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FrameBuffer {
    id: GLuint,
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
        let mut buffers = [Self { id: 0 }; N];
        let mut ids = [0; N];
        unsafe {
            gl::GenFramebuffers(N as i32, &mut ids[0] as *mut GLuint as *mut GLuint);
        }
        for (buffer, id) in buffers.iter_mut().zip(ids.iter()) {
            buffer.id = *id;
        }
        buffers
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
    id: GLuint,
}
