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

const QUAD: [f32; 3 * 2 * 2] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0];

const TEST_TILE: &'static str = "tiles/Spaceland.Space/C. Anomalies/anom-001.png";

struct HexGrid {
    size: (u32, u32),
    tile_size: (u32, u32),
    vbo: u32,
    vao: u32,
    texture: render::texture::Texture2D,
    program: render::program::Program,
}

const VERT: &str = r#"
#version 330
layout(location = 0) in vec2 pos;
uniform vec2 offset;
uniform vec2 size;
uniform mat4 projection;

out vec2 texpos;

void main() {
    gl_Position = projection * vec4(offset + pos * size, 1.0, 1.0);
    texpos = pos;
}
"#;

const FRAG: &str = r#"
#version 330
// uniform sampler2D character;
in vec2 texpos;

void main() {
  gl_FragColor = vec4(1.0, 0.0, 1.0, 0.5);
  // gl_FragColor = vec4(1.0) * texture2D(character, texpos).r;
}
"#;

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

        let rt = Runtime::new().unwrap();
        rt.spawn(other(event_loop.create_proxy()));

        let texture = render::texture::Texture2D::from_image(
            image::io::Reader::open(TEST_TILE)
                .unwrap()
                .decode()
                .unwrap(),
        );

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
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo as *mut GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                QUAD.len() as isize * ::std::mem::size_of::<f32>() as isize,
                &QUAD[0] as *const _ as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            0,
            ::std::ptr::null(),
        );


        event_loop.run(move |event, _, control_flow| {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            program.uniform_mat4("projection", &projection);
            program.uniform_vec2("offset", [10f32, 10f32].into());
            program.uniform_vec2("size", [100f32, 100f32].into());

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::DrawArrays(gl::TRIANGLES, 0, 6 as i32);
            context.swap_buffers().unwrap();
        });
    }
}
