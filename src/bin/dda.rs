extern crate minifb;

use minifb::{Window, WindowOptions};
use graficacion::algorithms::line_dda;
use graficacion::vertex::Vertex;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


fn main() {

    let mut buffer = vec![0; WIDTH*HEIGHT];
    let mut window = Window::new(
        "Digital Differential Analyzer",
        WIDTH,
        HEIGHT,
        WindowOptions{
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap();


    //set limit to 60 fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    let vertex1 = Vertex{position: [0.0, 0.0]};
    let vertex2 = Vertex{position: [639.0, 357.0]};

    let vertex3 = Vertex{position: [200.0, 300.0]};
    let vertex4 = Vertex{position: [400.0, 200.0]};

    let vertex5 = Vertex{position: [150.0, 150.0]};
    let vertex6 = Vertex{position: [150.0, 300.0]};

    while window.is_open() {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        line_dda(&vertex1, &vertex2, &mut buffer, WIDTH);

        line_dda(&vertex3, &vertex4, &mut buffer, WIDTH);

        line_dda(&vertex5, &vertex6, &mut buffer, WIDTH);

    }

}
