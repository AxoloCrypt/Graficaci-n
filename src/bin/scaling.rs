extern crate glium;

use glium::{glutin, Surface, uniform};
use graficacion::vertex::VertexF;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(600, 600))
        .with_title("Scaling Figures");

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let triangle_vertex_1 = VertexF{position: [0.0, 0.0]};
    let triangle_vertex_2 = VertexF{position: [0.5, 0.5]};
    let triangle_vertex_3 = VertexF{position: [1.0, 0.0]};

    let triangle = vec![triangle_vertex_1, triangle_vertex_2, triangle_vertex_3];
    let triangle_vertex_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();

    let triangle_vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main(){
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }

    "#;

    let triangle_fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main(){
            color = vec4(1.0, 0.5, 1.0, 1.0);
        }

    "#;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(&display, triangle_vertex_shader_src,
    triangle_fragment_shader_src, None).unwrap();


    let mut t: f32 = -0.5;


    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        t += 0.0012;
        // if t > 2.0 {
        //     t = -0.5;
        // }

        let matrix = [
            [t.cos(), t.sin(), 0.0, 0.0],
            [-t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let uniforms = uniform! {matrix: matrix};

        target.draw(&triangle_vertex_buffer, &indices, &program,
                    &uniforms,
                    &Default::default()).unwrap();

        target.finish().unwrap();


        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent {event, ..} => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },

                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached {..} => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            }

            _ => return,
        }
    })



}