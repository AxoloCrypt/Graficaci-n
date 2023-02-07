#[macro_use]
extern crate glium;

mod lib;


use glium::{glutin, Surface};
use lib::vertex::Vertex;


fn main() {

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display
    ::new(window_builder, context_builder, &event_loop).unwrap();

    let vertex1 = Vertex{position: [0.0, 0.5]};
    let vertex2 = Vertex{position: [-0.5, -0.5]};

    let shape = vec![vertex1, vertex2];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape)
        .unwrap();


    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }

    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main(){
            color = vec4(1.0, 0.0, 0.0, 1.0);

        }

    "#;


    let program = glium::Program
    ::from_source(&display,vertex_shader_src, fragment_shader_src,
    None).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();
        target.finish().unwrap();


        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);


        match ev {

            glutin::event::Event::WindowEvent {event, ..} => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });


}
