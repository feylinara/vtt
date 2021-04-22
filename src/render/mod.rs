pub mod program;
pub mod texture;
pub use self::program::{Program, ProgramBuilder, Shader, ShaderType};

use cgmath::{Matrix, Matrix4, SquareMatrix};
use gl;
use gl::types::GLuint;
use std::os::raw::c_void;

pub trait Bindable {
    fn bind(&self);
}

pub trait Buffer: Bindable {}

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
        let mut ids = [Self { id: 0 }; N];
        unsafe {
            gl::GenBuffers(N as i32, &mut ids[0] as *mut Self as *mut GLuint);
        }
        ids
    }
    pub fn data<T>(&self, data: &[T], freq: AccessFrequency, typ: AccessType) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.len() as isize * ::std::mem::size_of::<T>() as isize,
                &data[0] as *const _ as *const c_void,
                storage_type(freq, typ),
            );
        }
    }
}

impl Bindable for VertexBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
}

impl Buffer for VertexBuffer {}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VertexAttribObject {
    id: GLuint,
}

impl VertexAttribObject {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id as *mut GLuint);
        }
        Self { id }
    }
    pub fn new_array<const N: usize>() -> [Self; N] {
        let mut ids = [Self { id: 0 }; N];
        unsafe {
            gl::GenVertexArrays(N as i32, &mut ids[0] as *mut Self as *mut GLuint);
        }
        ids
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn vertex_attribute_array<T: GlType>(&self, buffer: &dyn Buffer, ptr: VertexAttribArray<T>) {
        self.bind();
        buffer.bind();
        unsafe {
            gl::EnableVertexAttribArray(ptr.id);
            ptr.divisor
                .map(|divisor| gl::VertexAttribDivisor(ptr.id, divisor));
            gl::VertexAttribPointer(
                ptr.id,
                ptr.ncomponents,
                T::to_enum(),
                if ptr.normalise { gl::TRUE } else { gl::FALSE },
                ptr.stride,
                ptr.offset as *mut c_void,
            );
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AccessFrequency {
    Static,
    Stream,
    Dynamic,
}

#[derive(Clone, Copy, Debug)]
pub enum AccessType {
    Draw,
    Read,
    Copy,
}

fn storage_type(freq: AccessFrequency, typ: AccessType) -> GLuint {
    use AccessFrequency::*;
    use AccessType::*;
    match (freq, typ) {
        (Static, Draw) => gl::STATIC_DRAW,
        (Static, Read) => gl::STATIC_READ,
        (Static, Copy) => gl::STATIC_COPY,
        (Stream, Draw) => gl::STREAM_DRAW,
        (Stream, Read) => gl::STREAM_READ,
        (Stream, Copy) => gl::STREAM_COPY,
        (Dynamic, Draw) => gl::DYNAMIC_DRAW,
        (Dynamic, Read) => gl::DYNAMIC_READ,
        (Dynamic, Copy) => gl::DYNAMIC_COPY,
    }
}

pub trait GlType {
    fn to_enum() -> GLuint;
}

impl GlType for f32 {
    fn to_enum() -> GLuint {
        gl::FLOAT
    }
}

pub struct VertexAttribArray<T: GlType> {
    id: u32,
    divisor: Option<u32>,
    ncomponents: i32,
    normalise: bool,
    stride: i32,
    offset: i32,
    _pd: std::marker::PhantomData<T>,
}

impl<T: GlType> VertexAttribArray<T> {
    pub fn with_id(id: u32) -> Self {
        Self {
            id,
            divisor: None,
            normalise: false,
            stride: 0,
            offset: 0,
            ncomponents: 1,
            _pd: std::marker::PhantomData,
        }
    }
    pub fn normalise(mut self) -> Self {
        self.normalise = true;
        self
    }
    pub fn with_divisor(mut self, divisor: u32) -> Self {
        self.divisor = Some(divisor);
        self
    }
    pub fn with_components_per_value(mut self, ncomponents: i32) -> Self {
        self.ncomponents = ncomponents;
        self
    }
    pub fn with_stride(mut self, stride: i32) -> Self {
        self.stride = stride;
        self
    }
    pub fn with_offset(mut self, offset: i32) -> Self {
        self.offset = offset;
        self
    }
}
