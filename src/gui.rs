use cgmath::Vector2;
use crate::fgl::texture::Texture2D;
use font_kit::loaders::default::Font as FKFont;
use harfbuzz_rs::Font as HBFont;
use font_kit::source::Source;

type GlyphId = u32;

pub struct GlyphCache<S: Source> {
    source: S,
    font_cache: std::collections::HashMap<String, (FKFont, HBFont)>,
    glyph_cache: std::collections::HashMap<(String, GlyphId), Texture2D>,
}

pub trait Widget {
    fn calculate_size() -> Vector2<f32>;
    fn draw(texture: &Texture2D, rect: crate::render::compose::Quad);
}

pub enum Span {
    Text {
        text: String,
        font: String,
    },
    Newline,
}

pub struct TextBox {
    text: Vec<Span>,
}

impl TextBox {
    fn draw(vao: &VertexAttribObject, program: &Program, glyph_cache: &GlyphCache) {
        
    }
}

