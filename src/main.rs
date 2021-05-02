mod hex;
use hex::grid::HexGridBuilder;
use hex::token::{CentredOn, Mask, Token, TokenInstance, TokenManager};

mod render;

use cgmath::{Matrix3, Matrix4, SquareMatrix, Vector2, Vector3, Vector4, Zero};
use glutin::{
    dpi::PhysicalPosition,
    event_loop::{EventLoop, EventLoopProxy},
    window::WindowBuilder,
    ContextBuilder,
};
use tokio::runtime::Runtime;

pub enum NetworkEvent {}

async fn other(_: EventLoopProxy<NetworkEvent>) {}

const TEST_TILE1: &str = "tiles/Spaceland.Space/C. Anomalies/anom-008.png";
const TEST_TILE2: &str = "tiles/Spaceland.Space/C. Anomalies/anom-004.png";

const TEST_TOKEN: &str = "mechs/HA GENGHIS.png";

const VERT: &str = include_str!("../resources/shaders/grid.vert");
const FRAG: &str = include_str!("../resources/shaders/grid.frag");

fn main() {
    let event_loop = EventLoop::with_user_event();
    let window_builder = WindowBuilder::new();
    let context = unsafe {
        ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
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

    let images = [
        image::io::Reader::open(TEST_TILE1)
            .unwrap()
            .decode()
            .unwrap(),
        image::io::Reader::open(TEST_TILE2)
            .unwrap()
            .decode()
            .unwrap(),
    ];

    let mut hex_grid = HexGridBuilder::default()
        .with_dimensions(50, 50)
        .point_up()
        .with_tiles(&images)
        .build();
    hex_grid.update_tile((2, 1), Some(1));

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

    let mut projection = cgmath::ortho(
        0f32,
        context.window().inner_size().width as f32,
        0f32,
        context.window().inner_size().height as f32,
        -1f32,
        100f32,
    );
    let mut scroll = Vector2::zero();
    let mut mouse_position = PhysicalPosition::new(0.0, 0.0);
    let mut drag = false;
    let mut scale = 0.5f32;

    let token_image = image::io::Reader::open(TEST_TOKEN)
        .unwrap()
        .decode()
        .unwrap();
    let (mut token_manger, token_ids) = TokenManager::new(
        210.0,
        std::array::IntoIter::new([Token::new(
            token_image,
            0,
            true,
            Mask::None,
            CentredOn::Corner { point_up: true },
        )]),
    )
    .unwrap();
    token_manger.append_instances(&[TokenInstance {
        coords: (3, 2).into(),
        token: token_ids[0],
    }]);

    let fb = crate::render::framebuffer::FrameBuffer::new();
    let rb = crate::render::framebuffer::RenderBuffer::new();
    rb.alloc(
        context.window().inner_size().width,
        context.window().inner_size().height,
        crate::render::framebuffer::Format::DepthStencil,
        0,
    );
    fb.attach_renderbuffer(&rb, crate::render::framebuffer::Attachment::DepthStencil);
    let t = render::texture::Texture2D::with_dimensions(
        context.window().inner_size().width as i32,
        context.window().inner_size().height as i32,
        crate::render::texture::Format::Rgba,
    );
    fb.attach_texture2d(&t, crate::render::framebuffer::Attachment::Color(0));

    event_loop.run(move |event, _, control_flow| unsafe {
        use glutin::event::{Event, MouseScrollDelta, WindowEvent};
        *control_flow = glutin::event_loop::ControlFlow::Wait;
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                WindowEvent::Resized(ps) => {
                    context.resize(ps);
                    projection =
                        cgmath::ortho(0f32, ps.width as f32, 0f32, ps.height as f32, -1f32, 100f32);
                    gl::Viewport(0, 0, ps.width as i32, ps.height as i32);
                }
                WindowEvent::MouseInput {
                    button: winit::event::MouseButton::Left,
                    state,
                    ..
                } => {
                    drag = state == winit::event::ElementState::Pressed;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if drag {
                        let scroll_by = Matrix4::from_nonuniform_scale(scale, scale, 1.0)
                            .invert()
                            .unwrap()
                            * Matrix4::from_translation(Vector3::new(
                                (position.x - mouse_position.x) as f32,
                                (mouse_position.y - position.y) as f32,
                                0.0,
                            ))
                            * Vector4::new(0.0, 0.0, 0.0, 1.0);
                        scroll += Vector2::new(scroll_by.x, scroll_by.y);
                    }
                    mouse_position = position;
                    context.window().request_redraw();
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            if scale >= 0.05 || y >= 0.0 {
                                let mouse = Vector2::new(
                                    mouse_position.x as f32,
                                    context.window().inner_size().height as f32
                                        - mouse_position.y as f32,
                                );
                                let new_scale = scale + y * 0.05;
                                scroll += -(Matrix3::from_scale(scale).invert().unwrap()
                                    * mouse.extend(1.0)
                                    - Matrix3::from_scale(new_scale).invert().unwrap()
                                        * mouse.extend(1.0))
                                .truncate();
                                scale = new_scale;
                            }
                        }
                        MouseScrollDelta::PixelDelta(pp) => {
                            println!("Gesture scroll not implemented {:?}", pp);
                        }
                    }
                    context.window().request_redraw();
                }
                _ => {}
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                gl::ClearColor(0.8, 0.8, 0.8, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                hex_grid.draw(
                    &program,
                    projection
                        * cgmath::Matrix4::from_nonuniform_scale(scale, scale, 1.0)
                        * cgmath::Matrix4::from_translation(Vector3::new(scroll.x, scroll.y, 0f32)),
                );
                fb.bind();
                token_manger.draw(
                    projection
                        * cgmath::Matrix4::from_nonuniform_scale(scale, scale, 1.0)
                        * cgmath::Matrix4::from_translation(Vector3::new(scroll.x, scroll.y, 0f32)),
                );
                fb.unbind();
                let mut err = gl::GetError();
                while err != gl::NO_ERROR {
                    println!(
                        "Uncaught OpenGl Error: {}",
                        match err {
                            gl::INVALID_ENUM => "invalid enum".to_string(),
                            gl::INVALID_VALUE => "invalid value".to_string(),
                            gl::INVALID_OPERATION => "invalid operation".to_string(),
                            x => format!("{}", x),
                        }
                    );
                    err = gl::GetError();
                }
                context.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    });
}
