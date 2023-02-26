/*
    *******************************************
    Library implementations for coding exercises
    Author: AxoloCrypt
    Date: 12/02/2023
    *******************************************
 */


pub mod vertex {

    use glium::implement_vertex;

    #[derive(Copy, Clone)]
    pub struct VertexI {
        pub position: [i32; 2]
    }

    #[derive(Copy, Clone)]
    pub struct VertexF {
       pub position: [f32; 2]
    }


    implement_vertex!(VertexF, position);
}

pub mod algorithms {
    use crate::vertex;

    use vertex::{VertexI, VertexF};


    // Implementation of Digital Differential Analyzer
    pub fn line_dda(start: &VertexF, end: &VertexF, buffer: &mut Vec<u32>, screen_width: usize) {

        let dx = end.position[0] - start.position[0];
        let dy = end.position[1] - start.position[1];
        let steps;

        let mut x = start.position[0];
        let mut y = start.position[1];


        if dx.abs() > dy.abs() {
            steps = dx.abs();
        }
        else {
            steps = dy.abs();
        }

        let x_increment = dx / steps;
        let y_increment = dy / steps;

        set_pixel_f(x.round(), y.round(), buffer, screen_width);

        let i_steps = steps as i32;

        for _k in 0..i_steps {
            x += x_increment;
            y += y_increment;

            set_pixel_f(x.round(), y.round(), buffer, screen_width);
        }

    }

    pub fn line_bresenham(start: &VertexI, end: &mut VertexI, buffer: &mut Vec<u32>, screen_width: usize){

        let dx = (end.position[0] - start.position[0]).abs();
        let dy = (end.position[1] - start.position[1]).abs();
        let mut p = 2 * dy - dx;
        let two_dy = 2 * dx;
        let two_dy_dx = 2 * (dy - dx);

        let mut x; let mut y;

        if start.position[0] > end.position[0] {
            x = end.position[0];
            y = end.position[1];
            end.position[0] = start.position[0];
        }
        else {
            x = start.position[0];
            y = start.position[1];
        }

        set_pixel_i(x, y, buffer, screen_width);

        while x < end.position[0] {

            x += 1;

            if p < 0 {
                p += two_dy;
            }
            else {
                y += 1;
                p += two_dy_dx;
            }

            set_pixel_i(x, y, buffer, screen_width);
        }

    }

    fn set_pixel_f(x: f32, y: f32, buffer: &mut Vec<u32>, screen_width : usize){
        let position = ((y as usize) * (screen_width)) + x as usize;
        buffer[position] = 0xff_ff_ff; //white
    }

    fn set_pixel_i(x: i32, y: i32, buffer: &mut Vec<u32>, screen_width: usize) {
        let position = ((y as usize) * (screen_width)) + x as usize;
        buffer[position] = 0xff_ff_ff; //white
    }


}

#[cfg(test)]
mod test{
    use crate::algorithms;
    use crate::vertex::VertexF;

    #[test]
    fn test_dda() {
        let vertex1 = VertexF {position: [0.5, 0.5]};
        let vertex2 = VertexF {position: [-0.5, -0.5]};

        //algorithms::line_dda(&vertex1, &vertex2);



    }

}
