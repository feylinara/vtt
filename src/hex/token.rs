use crate::fgl::{self, Bindable, Program};
use cgmath::{Vector2, Zero};
use image::{DynamicImage, GenericImageView};
use itertools::Itertools;
use fgl::{ProgramBuilder, Shader};

const VERT: &str = include_str!("../../resources/shaders/token.vert");
const FRAG: &str = include_str!("../../resources/shaders/token.frag");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenHandle(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CentredOn {
    Tile,
    Corner { point_up: bool },
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mask {
    None = 0,
    Behind = 1,
    Clip = 2,
}

#[repr(C)]
struct TokenUniform {
    size: Vector2<f32>,
    mask_size: Vector2<f32>,
    mask: Mask,
}

pub struct TokenManager {
    tile_size: f32,
    tokens: Vec<Token>,
    instances: Vec<TokenInstance>,
    vbos: [fgl::VertexBuffer; 2],
    vao: fgl::VertexAttribObject,
    masks: Vec<fgl::texture::Texture2D>,
    needs_update: bool,
    program: Program,
}

impl TokenManager {
    pub fn new(tile_size: f32, tokens: impl IntoIterator<Item=Token>) -> Result<(Self, Vec<TokenHandle>), String> {
        let vao = fgl::VertexAttribObject::new();
        let mut vbos: [fgl::VertexBuffer; 2] = fgl::VertexBuffer::new_array();

        vbos[0].alloc_with(
            &fgl::consts::QUAD,
            fgl::AccessFrequency::Static,
            fgl::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbos[0],
            fgl::VertexAttribArray::<f32>::with_id(0).with_components_per_value(2),
        );

        vao.vertex_attribute_array(
            &vbos[1],
            fgl::VertexAttribArray::<f32>::with_id(1)
                .with_components_per_value(2)
                .with_divisor(6),
        );

        let program = ProgramBuilder::default()
            .attach_shader(Shader::from_source(fgl::ShaderType::Fragment, FRAG)?)
            .attach_shader(Shader::from_source(fgl::ShaderType::Vertex, VERT)?)
            .link()?;

        let tokens: Vec<_> = tokens.into_iter().collect();
        let len = tokens.len();

        Ok((
            Self {
                tokens,
                instances: Vec::new(),
                vbos,
                vao,
                masks: Vec::new(),
                tile_size,
                needs_update: false,
                program,
            },
            (0..len).map(TokenHandle).collect(),
        ))
    }

    pub fn append_tokens(&mut self, tokens: impl IntoIterator<Item=Token>) -> Vec<TokenHandle> {
        let old_len = self.tokens.len();
        self.tokens.extend(tokens);
        (old_len..self.tokens.len()).map(TokenHandle).collect()
    }

    pub fn append_instances(&mut self, instances: &[TokenInstance]) {
        self.instances.extend_from_slice(instances);
        self.instances.sort_by_key(|instance| instance.token);
        let data: Vec<_> = self
            .instances
            .iter()
            .map(|x| {
                super::grid_to_world(x.coords, self.tile_size)
                    + match self.tokens[x.token.0].centred_on {
                        CentredOn::Tile => Vector2::zero(),
                        CentredOn::Corner { .. } => super::corner_offset(self.tile_size, 0),
                    }
            })
            .collect();
        self.vbos[1].alloc_with(
            &data,
            fgl::AccessFrequency::Dynamic,
            fgl::AccessType::Draw,
        );
    }

    pub fn find_instances_at(
        &mut self,
        coords: Vector2<u32>,
    ) -> impl Iterator<Item = &mut TokenInstance> {
        self.needs_update = true;
        self.instances
            .iter_mut()
            .filter(move |x| x.coords == coords)
    }

    pub fn update(&mut self) {
        if self.needs_update {
            self.instances.sort_by_key(|instance| instance.token);
            let data: Vec<_> = self
                .instances
                .iter()
                .map(|x| {
                    super::grid_to_world(x.coords, self.tile_size)
                        + match self.tokens[x.token.0].centred_on {
                            CentredOn::Tile => Vector2::zero(),
                            CentredOn::Corner { .. } => super::corner_offset(self.tile_size, 0),
                        }
                })
                .collect();
            self.vbos[1].replace_sub_data(0, &data);
        }
    }

    pub fn draw(&self, projection: cgmath::Matrix4<f32>) {
        self.vao.bind();
        self.program.bind();
        let batches: Vec<_> = self
            .instances
            .iter()
            .group_by(|instance| instance.token)
            .into_iter()
            .map(|g| (g.0, g.1.count()))
            .collect();
        self.program.uniform_mat4("projection", &projection);
        let mut first = 0;
        for (handle, batch_size) in batches {
            let token = &self.tokens[handle.0];
            self.program.uniform_vec2(
                "dimensions",
                if token.scale {
                    let Vector2 { x, y } = token.dimensions;
                    (if x > y {
                        Vector2::new(1.0, y as f32 / x as f32)
                    } else {
                        Vector2::new(x as f32 / y as f32, 1.0)
                    }) * self.tile_size
                } else {
                    token.dimensions.map(|x| x as f32)
                },
            );
            token.texture.bind(0);
            self.program.uniform_i32("token", 0);
            unsafe {
                gl::DrawArraysInstanced(
                    gl::TRIANGLES,
                    first * 6,
                    6i32,
                    (batch_size * 6) as i32,
                );
            }
            first += batch_size as i32;
        }
    }
}

pub struct Token {
    texture: fgl::texture::Texture2D,
    dimensions: Vector2<u32>,
    nominal_size: u32,
    scale: bool,
    mask: Mask,
    centred_on: CentredOn,
}

impl Token {
    pub fn new(
        image: DynamicImage,
        nominal_size: u32,
        scale: bool,
        mask: Mask,
        centred_on: CentredOn,
    ) -> Self {
        Self {
            dimensions: image.dimensions().into(),
            texture: crate::fgl::texture::Texture2D::from_image(image),
            nominal_size,
            scale,
            mask,
            centred_on,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TokenInstance {
    pub coords: Vector2<u32>,
    pub token: TokenHandle,
}
