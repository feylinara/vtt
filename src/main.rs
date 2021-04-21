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
}

impl<'a> Default for HexGridBuilder<'a> {
    fn default() -> Self {
        Self {
            outer_radius: 0, point_up: false, tiles: &[],
        }
    }
}

impl<'a> HexGridBuilder<'a> {
    fn build(self) -> HexGrid {
        let mut texture = render::texture::Texture2D::with_dimensions(
            210 * self.tiles.len() as i32,
            210,
            render::texture::Format::Rgba,
        );

        for image in self.tiles.iter() {
            texture.replace_rect(0, 0, image.clone());
        }

        let mut vbo = [0u32; 3];
        unsafe {
            gl::GenBuffers(vbo.len() as i32, &mut vbo[0] as *mut GLuint);
        }
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
    program: render::program::Program,
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
    unsafe {
        let event_loop = EventLoop::with_user_event();
        let window = WindowBuilder::new();
        let context = ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        gl::load_with(|s| context.get_proc_address(s));
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

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
        let mut texture = render::texture::Texture2D::with_dimensions(
            210 * 2,
            210,
            render::texture::Format::Rgba,
        );
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

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao as *mut GLuint);
            gl::BindVertexArray(vao);
        }
        let mut vbo = [0; 3];
        unsafe {
            gl::GenBuffers(vbo.len() as i32, &mut vbo[0] as *mut GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo[0]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                QUAD.len() as isize * ::std::mem::size_of::<f32>() as isize,
                &QUAD[0] as *const _ as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ::std::ptr::null());

        let offsets = grid_coords(4, 4, 210f32);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo[1]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            offsets.len() as isize * ::std::mem::size_of::<f32>() as isize,
            &offsets[0] as *const _ as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribDivisor(1, 6);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, ::std::ptr::null());

        let tiles = [
            0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
            0f32, 0f32,
        ];
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo[2]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            tiles.len() as isize * ::std::mem::size_of::<f32>() as isize,
            &tiles[0] as *const _ as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribDivisor(2, 6);
        gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, 0, ::std::ptr::null());

        event_loop.run(move |event, _, control_flow| {
            draw(&program, projection, vao, &context);
        });
    }
}

unsafe fn draw(
    program: &render::Program,
    projection: cgmath::Matrix4<f32>,
    vao: u32,
    context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
) {
    gl::ClearColor(0.8, 0.8, 0.8, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    program.bind();
    program.uniform_mat4("projection", &projection);
    program.uniform_vec2("size", [210f32, 210f32].into());
    program.uniform_f32("ntiles", 2.0);
    gl::BindVertexArray(vao);
    gl::DrawArraysInstanced(gl::TRIANGLES, 0, 6 as i32, 6 * 16);
    context.swap_buffers().unwrap();
}
