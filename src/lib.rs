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
    pub struct Vertex {
       pub position: [f32; 2]
    }

    implement_vertex!(Vertex, position);
}

pub mod algorithms {
    use crate::vertex;

    use vertex::Vertex;


    // Implementation of Digital Differential Analyzer
    pub fn line_dda(start: &Vertex, end: &Vertex, buffer: &mut Vec<u32>, screen_width: usize) {

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

        set_pixel(x.round(), y.round(), buffer, screen_width);

        let i_steps = steps as i32;

        for _k in 0..i_steps {
            x += x_increment;
            y += y_increment;

            set_pixel(x.round(), y.round(), buffer, screen_width);
        }

    }

    fn set_pixel(x: f32, y: f32, buffer: &mut Vec<u32>, screen_width : usize){
        let position = ((y as usize) * (screen_width)) + x as usize;
        buffer[position] = 0xff_ff_ff;
    }


}

#[cfg(test)]
mod test{
    use crate::algorithms;
    use crate::vertex::Vertex;

    #[test]
    fn test_dda() {
        let vertex1 = Vertex{position: [0.5, 0.5]};
        let vertex2 = Vertex{position: [-0.5, -0.5]};

        //algorithms::line_dda(&vertex1, &vertex2);



    }

}
