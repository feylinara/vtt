pub mod program;
pub mod texture;
pub use self::program::{Program, ProgramBuilder, Shader, ShaderType};

use cgmath::{Matrix, Matrix4, SquareMatrix};
use gl;
use gl::types::GLuint;
use std::os::raw::c_void;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VertexBuffer {
    id: GLuint,
}

impl VertexBuffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id as *mut GLuint);
        }
        Self { id }
    }
    pub fn new_array<const N: usize>() -> [Self; N] {
        let mut ids = [Self {id: 0}; N];
        unsafe {
            gl::GenBuffers(N as i32, &mut ids[0] as *mut Self as *mut GLuint);
        }
        ids
    }

    pub fn bind(&self) {
        unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
}