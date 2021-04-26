use itertools::Itertools;
use crate::render;

const QUAD: [f32; 3 * 2 * 2] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];

pub struct TokenHandle(usize);

pub enum CentredOn {
    Tile,
    Corner,
}

#[repr(u8)]
pub enum Mask {
    None = 0,
    Behind = 1,
    Clip = 2,
}

#[repr(C)]
struct TokenUniform {
    size: (f32, f32),
    mask_size: (f32, f32),
    mask: Mask,
}

pub struct TokenManager {
    tokens: Vec<Token>,
    instances: Vec<TokenInstance>,
    vbos: [render::VertexBuffer; 2],
    vao: render::VertexAttribObject,
    masks: Vec<crate::render::texture::Texture2D>,
}

impl TokenManager {
    pub fn new(tokens: &[Token]) -> Vec<TokenHandle> {
        let vao = render::VertexAttribObject::new();
        let mut vbos: [render::VertexBuffer; 3] = render::VertexBuffer::new_array();
        vbos[0].alloc_with(
            &QUAD,
            render::AccessFrequency::Static,
            render::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbos[0],
            render::VertexAttribArray::<f32>::with_id(0).with_components_per_value(2),
        );

        (0..tokens.len()).map(|n| {TokenHandle(n)}).collect()
    }

    pub fn append_tokens(tokens: &[Token]) -> Vec<TokenHandle> {
        unimplemented!()
    }
    pub fn append_instances(tokens: &[TokenInstance]) {
        unimplemented!()
    }

    pub fn find_instances_at(coords: (u32, u32)) -> impl Iterator<&mut TokenInstance> {
        unimplemented!()
    }

    pub fn draw() -> impl Iterator<&mut TokenInstance> {
        unimplemented!()
    }
}

pub struct Token {
    texture: crate::render::texture::Texture2D,
    dimensions: (u32, u32),
    nominal_size: u32,
    scale: bool,
    mask: Mask,
    centred_on: CentredOn,
}

pub struct TokenInstance {
    coords: (u32, u32),
    token: TokenHandle,
}
