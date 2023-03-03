extern crate glium;

use glium::{glutin, Surface, uniform};
use graficacion::vertex::{Normal3D, Vertex3D};

fn main() {

    //Build WindowBuilder, ContextBuilder, EventLoop and Display
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800, 800))
        .with_title("Rotating Cube");

    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);


    let display = glium::Display::new(wb, cb, &event_loop)
        .unwrap();

    // Set cube figure

    let cube = [
        //front
        Vertex3D { position: (-0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5,  -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5,  0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        // right
        Vertex3D { position: (0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        // back
        Vertex3D { position: (0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        //left
        Vertex3D { position: (-0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        //top
        Vertex3D { position: (-0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, 0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        //bottom
        Vertex3D { position: (-0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (-0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, -0.5, -0.8), normal: (0.0, 0.0, -1.0) },
        Vertex3D { position: (0.5, -0.5, 0.0), normal: (0.0, 0.0, -1.0) },
    ];

    let cube_normals = [
        Normal3D {position: (0.0, 0.0, -1.0)},
        Normal3D {position: (0.0, 0.0, -1.0)},
        Normal3D {position: (0.0, 0.0, -1.0)},
        Normal3D {position: (0.0, 0.0, -1.0)},
    ];

    // let cube = [
    //     // front
    //    Vertex3D3D{position: (-0.5, -0.5, 0.5)}, Vertex3D3D{position: (0.5, -0.5, 0.5)}, Vertex3D3D{position: (-0.5, 0.5, 0.5)}, Vertex3D3D{position: (-0.5, 0.5, 0.5)}, Vertex3D3D{position: (0.5, -0.5, 0.5)}, Vertex3D3D{position: (0.5, 0.5, 0.5)},
    //     //right
    //     Vertex3D3D{position: (0.5, -0.5, 0.5)},Vertex3D3D{position: (0.5, -0.5, -0.5)}, Vertex3D3D{position: (0.5, 0.5, 0.5)}, Vertex3D3D{position: (0.5, 0.5, 0.5)}, Vertex3D3D{position: (0.5, -0.5, -0.5)}, Vertex3D3D{position: (0.5, 0.5, -0.5)},
    //     //back
    //     Vertex3D3D{position: (0.5, -0.5, -0.5)}, Vertex3D3D{position: (-0.5, -0.5, -0.5)}, Vertex3D3D{position: (0.5, 0.5, -0.5)}, Vertex3D3D{position: (0.5, 0.5, -0.5)}, Vertex3D3D{position: (-0.5, -0.5, -0.5)}, Vertex3D3D{position: (-0.5, 0.5, -0.5)},
    //     //left
    //     Vertex3D3D{position: (-0.5, -0.5, -0.5)}, Vertex3D3D{position: (-0.5, -0.5, 0.5)}, Vertex3D3D{position: (-0.5, 0.5, -0.5)}, Vertex3D3D{position: (-0.5, 0.5, -0.5)}, Vertex3D3D{position: (0.5, 0.5, 0.5)}, Vertex3D3D{position: (-0.5, 0.5, 0.5)},
    //     //top
    //     Vertex3D3D{position: (-0.5, 0.5, 0.5)}, Vertex3D3D{position: (0.5, 0.5, 0.5)}, Vertex3D3D{position: (-0.5, 0.5, -0.5)}, Vertex3D3D{position: (-0.5, 0.5, -0.5)}, Vertex3D3D{position: (0.5, 0.5, 0.5)}, Vertex3D3D{position: (0.5, 0.5, -0.5)},
    //     //bottom
    //     Vertex3D3D{position: (-0.5, -0.5, -0.5)}, Vertex3D3D{position: (0.5, -0.5, -0.5)}, Vertex3D3D{position: (-0.5, -0.5, 0.5)}, Vertex3D3D{position: (-0.5, -0.5, 0.5)}, Vertex3D3D{position: (0.5, -0.5, -0.5)}, Vertex3D3D{position: (0.5, -0.5, 0.5)}
    // ];

    // let cube_normals = [
    //     // front
    //     Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)},
    //     //right
    //     Normal3D{position: (0.0, 0.0, -1.0)},Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)},
    //     //back
    //     Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)},
    //     //left
    //     Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)},
    //     //top
    //     Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)},
    //     //bottom
    //     Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}, Normal3D{position: (0.0, 0.0, -1.0)}
    // ];

    // let indices = [
    //     1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    //     21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37u16
    // ];


    let cube_vertex_buffer = glium::VertexBuffer::new(&display, &cube).unwrap();
    let normals_vertex_buffer = glium::VertexBuffer::new(&display, &cube_normals).unwrap();
    // let cube_indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
    // &indices).unwrap();

    let cube_vertex_shader = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;
        uniform mat4 perspective;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;


    let cube_fragment_shader = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main(){
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.2, 0.2, 0.2);
            vec3 regular_color = vec3(1.0, 0.70, 0.65);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let cube_program = glium::Program::from_source(&display, cube_vertex_shader,
    cube_fragment_shader, None).unwrap();


    // let matrix = [
    //     [1.0, 0.0, 0.0, 0.0],
    //     [0.0, 1.0, 0.0, 0.0],
    //     [0.0, 0.0, 1.0, 0.0],
    //     [0.0, 0.0, 0.0, 1.0f32]
    // ];



    let light = [-1.0, 0.4, 0.9f32];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };


    let mut t:f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);


        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0f32).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
                [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0]
            ]
        };

        let matrix = [
            [t.cos(), t.sin(), 0.0, 0.0],
            [-t.sin(), t.cos(), t.sin(), 0.0],
            [0.0, -t.sin(),t.cos(), 0.0],
            [0.0, 0.0, 4.0, 1.0f32]
        ];

        let uniforms = uniform! {matrix : matrix, u_light: light, perspective: perspective};

        t += 0.0012;

        target.draw(&cube_vertex_buffer, &indices, &cube_program,
        &uniforms, &params).unwrap();

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