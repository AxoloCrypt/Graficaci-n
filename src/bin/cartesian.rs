extern crate glium;

use glium::{glutin, Surface, uniform};
use graficacion::vertex::VertexF;


fn main(){

    let mut events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(500, 450))
        .with_title("Cartesian Plane");

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();


    let vertex_x_1 = VertexF{position: [-1.0, 0.0]};
    let vertex_x_2 = VertexF{position: [1.0, 0.0]};
    let vertex_y_1 = VertexF{position: [0.0, 1.0]};
    let vertex_y_2 = VertexF{position: [0.0, -1.0]};

    let cartesian_plane = vec![vertex_x_1, vertex_x_2, vertex_y_1, vertex_y_2];

    let vertex_buffer = glium::VertexBuffer::new(&display, &cartesian_plane).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
    let triangle_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let triangle_vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float t;

        void main(){
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }

    "#;

    let triangle_fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main(){
            color = vec4(1.0, 0.5, 1.0, 1.0);
        }

    "#;

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main(){
            gl_Position = vec4(position, 0.0, 1.0);
        }

    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main(){
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }

    "#;

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src, fragment_shader_src,
                                              None).unwrap();

    let triangle_vertex_1 = VertexF{position: [-0.5, 0.65]};
    let triangle_vertex_2 = VertexF{position: [-0.75, 0.65]};
    let triangle_vertex_3 = VertexF{position: [-0.75, 0.85]};

    let triangle = vec![triangle_vertex_1, triangle_vertex_2, triangle_vertex_3];

    let triangle_vertex_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();

    let mut t: f32 = -0.5;

    let triangle_program = glium::Program::from_source(&display,
                                                       triangle_vertex_shader_src, triangle_fragment_shader_src, None).unwrap();
    events_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        t += 0.0012;
        if t > 2.0 {
            t = -0.5;
        }

        let uniforms = uniform! {t: t};

        target.draw(&triangle_vertex_buffer, &triangle_indices, &triangle_program,
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