mod render;

use gl::types::*;
use glutin::{
    event_loop::{EventLoop, EventLoopProxy},
    window::WindowBuilder,
    ContextBuilder,
};
use std::os::raw::c_void;
use tokio::runtime::Runtime;

pub enum NetworkEvent {}

async fn other(_: EventLoopProxy<NetworkEvent>) {}

struct HexGridBuilder<'a> {
    outer_radius: u32,
    point_up: bool,
    tiles: &'a [image::DynamicImage],
    dimensions: (u32, u32),
}

impl<'a> Default for HexGridBuilder<'a> {
    fn default() -> Self {
        Self {
            outer_radius: 0,
            point_up: false,
            tiles: &[],
            dimensions: (0, 0),
        }
    }
}

impl<'a> HexGridBuilder<'a> {
    fn with_tiles(mut self, tiles: &'a [image::DynamicImage]) -> Self {
        self.tiles = tiles;
        self
    }
    fn build(self) -> HexGrid {
        let mut texture = render::texture::Texture2D::with_dimensions(
            210 * self.tiles.len() as i32,
            210,
            render::texture::Format::Rgba,
        );

        for image in self.tiles.iter() {
            texture.replace_rect(0, 0, image.clone());
        }

        let mut vao = render::VertexAttribObject::new();

        let mut vbo: [render::VertexBuffer; 3] = render::VertexBuffer::new_array();
        vbo[0].data(
            &QUAD,
            render::AccessFrequency::Static,
            render::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbo[0],
            render::VertexAttribArray::<f32>::with_id(0).with_components_per_value(2),
        );

        let offsets = grid_coords(4, 4, 210f32);
        vbo[1].data(
            &offsets,
            render::AccessFrequency::Static,
            render::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbo[1],
            render::VertexAttribArray::<f32>::with_id(1)
                .with_components_per_value(2)
                .with_divisor(6),
        );

        let tiles = [
            0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
            0f32, 0f32,
        ];
        vbo[2].data(
            &tiles,
            render::AccessFrequency::Dynamic,
            render::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbo[2],
            render::VertexAttribArray::<f32>::with_id(2)
                .with_components_per_value(2)
                .with_divisor(6),
        );
        unimplemented!()
    }
}

const QUAD: [f32; 3 * 2 * 2] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];

const TEST_TILE1: &'static str = "tiles/Spaceland.Space/C. Anomalies/anom-008.png";
const TEST_TILE2: &'static str = "tiles/Spaceland.Space/C. Anomalies/anom-004.png";

struct HexGrid {
    size: (u32, u32),
    tile_size: (u32, u32),
    vbo: [u32; 3],
    vao: u32,
    texture: render::texture::Texture2D,
}

const VERT: &str = include_str!("../resources/shaders/grid.vert");
const FRAG: &str = include_str!("../resources/shaders/grid.frag");

fn grid_coords(height: u32, width: u32, tile_size: f32) -> Vec<f32> {
    let stepx = tile_size / 2f32 * 3f32.sqrt();
    let stepy = stepx * 5.0 / 6.0;
    let mut grid = Vec::new();
    for row in 0..height {
        for i in 0..width {
            grid.push(i as f32 * stepx - if row % 2 == 0 { stepx / 2.0 } else { 0.0 });
            grid.push(row as f32 * stepy);
        }
    }
    grid
}

fn main() {
    let event_loop = EventLoop::with_user_event();
    let window = WindowBuilder::new();
    let context = unsafe {
        ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };
    unsafe {
        gl::load_with(|s| context.get_proc_address(s));
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let rt = Runtime::new().unwrap();
    rt.spawn(other(event_loop.create_proxy()));

    let image1 = image::io::Reader::open(TEST_TILE1)
        .unwrap()
        .decode()
        .unwrap();
    let image2 = image::io::Reader::open(TEST_TILE2)
        .unwrap()
        .decode()
        .unwrap();
    let mut texture =
        render::texture::Texture2D::with_dimensions(210 * 2, 210, render::texture::Format::Rgba);
    texture.replace_rect(0, 0, image2.clone());
    texture.replace_rect(210, 0, image1.clone());

    let projection = cgmath::ortho(
        0f32,
        context.window().inner_size().width as f32,
        0f32,
        context.window().inner_size().height as f32,
        -1f32,
        100f32,
    );

    let program = render::program::ProgramBuilder::default()
        .attach_shader(
            render::program::Shader::from_source(render::program::ShaderType::Vertex, VERT)
                .unwrap(),
        )
        .attach_shader(
            render::program::Shader::from_source(render::program::ShaderType::Fragment, FRAG)
                .unwrap(),
        )
        .link()
        .unwrap();

    let mut vao = render::VertexAttribObject::new();

    let mut vbo: [render::VertexBuffer; 3] = render::VertexBuffer::new_array();
    vbo[0].data(
        &QUAD,
        render::AccessFrequency::Static,
        render::AccessType::Draw,
    );
    vao.vertex_attribute_array(
        &vbo[0],
        render::VertexAttribArray::<f32>::with_id(0).with_components_per_value(2),
    );

    let offsets = grid_coords(4, 4, 210f32);
    vbo[1].data(
        &offsets,
        render::AccessFrequency::Static,
        render::AccessType::Draw,
    );
    vao.vertex_attribute_array(
        &vbo[1],
        render::VertexAttribArray::<f32>::with_id(1)
            .with_components_per_value(2)
            .with_divisor(6),
    );

    let tiles = [
        0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
        0f32,
    ];
    vbo[2].data(
        &tiles,
        render::AccessFrequency::Dynamic,
        render::AccessType::Draw,
    );
    vao.vertex_attribute_array(
        &vbo[2],
        render::VertexAttribArray::<f32>::with_id(2)
            .with_components_per_value(2)
            .with_divisor(6),
    );

    event_loop.run(move |event, _, control_flow| unsafe {
        draw(&program, projection, vao, &context);
    });
}

unsafe fn draw(
    program: &render::Program,
    projection: cgmath::Matrix4<f32>,
    vao: render::VertexAttribObject,
    context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
) {
    gl::ClearColor(0.8, 0.8, 0.8, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    program.bind();
    program.uniform_mat4("projection", &projection);
    program.uniform_vec2("size", [210f32, 210f32].into());
    program.uniform_f32("ntiles", 2.0);
    vao.bind();
    gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6 as i32, 6 * 16);
    context.swap_buffers().unwrap();
}
