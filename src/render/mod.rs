pub mod program;
pub mod texture;
pub use self::program::{Program, ProgramBuilder, Shader, ShaderType};

use cgmath::{Matrix, Matrix4, SquareMatrix};
use gl;
use gl::types::GLuint;
use std::os::raw::c_void;

pub struct VertexBuffer {
    id: GLuint,
}
