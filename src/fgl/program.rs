use cgmath::{Array, Matrix, Matrix3, Matrix4, Vector3, Vector4, Vector2};
use gl;
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CString;

#[derive(Default)]
pub struct ProgramBuilder {
    shaders: Vec<Shader>,
}

/// An OpenGL Program Object
pub struct Program {
    pub(in crate::fgl) id: GLuint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl From<ShaderType> for GLenum {
    fn from(shader_type: ShaderType) -> GLenum {
        match shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    pub(in crate::fgl) id: GLuint,
}

impl Shader {
    pub fn from_source(shader_type: ShaderType, source: &str) -> Result<Self, String> {
        let shader = unsafe { gl::CreateShader(shader_type.into()) };
        let lengths = [source.as_bytes().len() as GLint];
        let source_ptr = &source.as_bytes()[0] as *const u8 as *const GLchar;
        unsafe {
            gl::ShaderSource(
                shader,
                1,
                &source_ptr as *const *const GLchar,
                &lengths[0] as *const GLint,
            );
            gl::CompileShader(shader);
        }

        let mut result = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result as *mut GLint);
        }
        if result != i32::from(gl::TRUE) {
            return Err(shader_info_log(shader));
        }
        Ok(Self { id: shader })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl ProgramBuilder {
    pub fn attach_shader(mut self, shader: Shader) -> Self {
        self.shaders.push(shader);
        self
    }

    pub fn link(self) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in &self.shaders {
                gl::AttachShader(id, shader.id);
            }

            gl::LinkProgram(id);

            for shader in &self.shaders {
                gl::DetachShader(id, shader.id);
            }

            let mut result = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut result as *mut GLint);
            if result != i32::from(gl::TRUE) {
                return Err(program_info_log(id));
            }
        }

        Ok(Program { id })
    }
}

impl Program {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    unsafe fn get_uniform_location(&self, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        gl::GetUniformLocation(self.id, name.as_ptr())
    }
    pub fn uniform_mat3(&self, name: &str, uniform: &Matrix3<f32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::UniformMatrix3fv(uniform_id, 1, gl::FALSE, uniform.as_ptr());
        }
    }
    pub fn uniform_mat4(&self, name: &str, uniform: &Matrix4<f32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::UniformMatrix4fv(uniform_id, 1, gl::FALSE, uniform.as_ptr());
        }
    }
    pub fn uniform_vec2(&self, name: &str, uniform: Vector2<f32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform2fv(uniform_id, 1, uniform.as_ptr());
        }
    }
    pub fn uniform_vec3(&self, name: &str, uniform: Vector3<f32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform3fv(uniform_id, 1, uniform.as_ptr());
        }
    }
    pub fn uniform_vec4(&self, name: &str, uniform: Vector4<f32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform4fv(uniform_id, 1, uniform.as_ptr());
        }
    }
    pub fn uniform_ivec2(&self, name: &str, uniform: Vector2<i32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform2iv(uniform_id, 1, uniform.as_ptr());
        }
    }
    pub fn uniform_ivec3(&self, name: &str, uniform: Vector3<i32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform3iv(uniform_id, 1, uniform.as_ptr());
        }
    }
    pub fn uniform_ivec4(&self, name: &str, uniform: Vector4<i32>) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform4iv(uniform_id, 1, uniform.as_ptr());
        }
    }
    pub fn uniform_i32(&self, name: &str, uniform: i32) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform1iv(uniform_id, 1, &uniform);
        }
    }
    pub fn uniform_f32(&self, name: &str, uniform: f32) {
        unsafe {
            let uniform_id = self.get_uniform_location(name);
            gl::Uniform1fv(uniform_id, 1, &uniform);
        }
    }
}

fn shader_info_log(shader: GLuint) -> String {
    const BUFFER_LEN: usize = 256;
    let mut error: Vec<u8> = Vec::new();
    let mut buffer: [u8; BUFFER_LEN] = [0; BUFFER_LEN];
    let mut actual_len: i32 = BUFFER_LEN as i32;

    while actual_len == BUFFER_LEN as i32 {
        unsafe {
            gl::GetShaderInfoLog(
                shader,
                BUFFER_LEN as i32,
                &mut actual_len as *mut i32,
                &mut buffer[0] as *mut _ as *mut GLchar,
            );
        }

        if actual_len == BUFFER_LEN as i32 {
            error.extend_from_slice(&buffer[..]);
        } else {
            for i in 0..actual_len {
                error.push(buffer[i as usize]);
            }
        }
    }

    String::from_utf8(error).unwrap()
}

fn program_info_log(program: GLuint) -> String {
    const BUFFER_LEN: usize = 256;
    let mut error: Vec<u8> = Vec::new();
    let mut buffer: [u8; BUFFER_LEN] = [0; BUFFER_LEN];
    let mut actual_len: i32 = BUFFER_LEN as i32;

    while actual_len == BUFFER_LEN as i32 {
        unsafe {
            gl::GetProgramInfoLog(
                program,
                BUFFER_LEN as i32,
                &mut actual_len as *mut i32,
                &mut buffer[0] as *mut _ as *mut GLchar,
            );
        }

        if actual_len == BUFFER_LEN as i32 {
            error.extend_from_slice(&buffer[..]);
        } else {
            for i in 0..actual_len {
                error.push(buffer[i as usize]);
            }
        }
    }

    String::from_utf8(error).unwrap()
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
