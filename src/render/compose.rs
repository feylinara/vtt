use cgmath::{Matrix4, Vector2, Zero};
use fgl::Shader;

use crate::fgl::{
    self, texture::Texture2D, AccessFrequency, AccessType, Bindable, Program, ProgramBuilder,
    VertexAttribArray, VertexAttribObject, VertexBuffer,
};
type WidgetId = u32;

macro_rules! include_shader {
    ($name:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/shaders/",
            $name
        ))
    };
}

const VERT: &str = include_shader!("compose.vert");
const FRAG: &str = include_shader!("compose.frag");

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Quad {
    pub offset: Vector2<u32>,
    pub size: Vector2<u32>,
}

impl Quad {
    fn contains(&self, vec: Vector2<u32>) -> bool {
        vec.x >= self.offset.x
            && vec.y >= self.offset.y
            && vec.x < (self.offset + self.size).x
            && vec.y < (self.offset + self.size).y
    }
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            offset: Zero::zero(),
            size: Zero::zero(),
        }
    }
}
pub struct QuadComposer {
    quads: Vec<(WidgetId, Quad)>,
    vbo: VertexBuffer,
    vao: VertexAttribObject,
    program: Program,
    projection: Matrix4<f32>,
}

impl QuadComposer {
    pub fn new(size: Vector2<u32>) -> Self {
        let program = ProgramBuilder::default()
            .attach_shader(Shader::from_source(fgl::ShaderType::Fragment, FRAG).unwrap())
            .attach_shader(Shader::from_source(fgl::ShaderType::Vertex, VERT).unwrap())
            .link()
            .unwrap();
        let mut vbo = VertexBuffer::new();
        vbo.alloc_with(
            &fgl::consts::QUAD,
            AccessFrequency::Static,
            AccessType::Draw,
        );
        let vao = VertexAttribObject::new();
        vao.vertex_attribute_array(
            &vbo,
            VertexAttribArray::<f32>::with_id(0).with_components_per_value(2),
        );
        let projection = cgmath::ortho(0f32, size.x as f32, 0f32, size.y as f32, -1f32, 100f32);
        Self {
            quads: Vec::new(),
            vbo,
            vao,
            program,
            projection,
        }
    }

    pub fn render_quad(&mut self, id: WidgetId, quad: Quad, texture: &Texture2D) {
        self.quads.push((id, quad));
        self.program.bind();
        texture.bind(0);
        self.program.uniform_i32("texture", 0);
        self.program.uniform_vec2("offset", quad.offset.map(|x| x as f32));
        self.program.uniform_vec2("dimensions", quad.size.map(|x| x as f32));
        self.program.uniform_mat4("projection", &self.projection);
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub fn resize(&mut self, size: Vector2<u32>) {
        self.projection = cgmath::ortho(0f32, size.x as f32, 0f32, size.y as f32, -1f32, 100f32);
    }

    pub fn resolve_click(&self, offset: Vector2<u32>) -> Option<WidgetId> {
        self.quads
            .iter()
            .rfind(|(_, quad)| quad.contains(offset))
            .map(|(id, _)| *id)
    }

    /// Discards click info
    pub fn end_frame(&mut self) {
        self.quads.clear()
    }
}
