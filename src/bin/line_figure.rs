extern crate minifb;

use minifb::{Window, WindowOptions};
use graficacion::algorithms::{line_dda, line_bresenham};
use graficacion::vertex::{VertexF, VertexI};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


fn main() {

    let mut buffer = vec![0; WIDTH*HEIGHT];
    let mut second_buffer = vec![0; WIDTH*HEIGHT];

    let mut window = Window::new(
        "Digital Differential Analyzer",
        WIDTH,
        HEIGHT,
        WindowOptions{
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap();

    let mut second_window = Window::new(
      "Bresenham Algorithm",
        WIDTH,
        HEIGHT,
        WindowOptions{
            resize: true,
            ..WindowOptions::default()
        }
    ).unwrap();



    //set limit to 60 fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    let vertex1 = VertexF{position: [150.0, 150.0]};
    let vertex2 = VertexF{position: [250.0, 50.0]};
    let vertex3 = VertexF{position: [400.0, 50.0]};
    let vertex4 = VertexF{position: [490.0, 150.0]};
    let vertex5 = VertexF{position: [400.0, 250.0]};
    let vertex6 = VertexF{position: [250.0, 250.0]};

    let vertex1_i = VertexI{position: [150, 150]};
    let mut vertex2_i = VertexI{position: [300, 250]};
    let mut vertex3_i = VertexI{position: [350, 150]};


    while window.is_open() {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        second_window.update_with_buffer(&second_buffer, WIDTH, HEIGHT).unwrap();

        line_dda(&vertex1, &vertex2, &mut buffer, WIDTH);
        line_dda(&vertex2, &vertex3, &mut buffer, WIDTH);
        line_dda(&vertex3, &vertex4, &mut buffer, WIDTH);
        line_dda(&vertex4, &vertex5, &mut buffer, WIDTH);
        line_dda(&vertex5, &vertex6, &mut buffer, WIDTH);
        line_dda(&vertex6, &vertex1, &mut buffer, WIDTH);
        line_dda(&vertex1, &vertex3, &mut buffer, WIDTH);
        line_dda(&vertex1, &vertex4, &mut buffer, WIDTH);
        line_dda(&vertex1, &vertex5, &mut buffer, WIDTH);
        line_dda(&vertex6, &vertex3, &mut buffer, WIDTH);
        line_dda(&vertex6, &vertex4, &mut buffer, WIDTH);
        line_dda(&vertex5, &vertex2, &mut buffer, WIDTH);
        line_dda(&vertex4, &vertex2, &mut buffer, WIDTH);

        line_bresenham(&vertex1_i, &mut vertex2_i, &mut second_buffer, WIDTH);
        line_bresenham(&vertex1_i, &mut vertex3_i, &mut second_buffer, WIDTH);
    }

}
