mod rotation;
mod display;
use std::{time, thread};
use std::io::{Write, stdout};

fn main() {
    let mut points: Vec<rotation::Point> = Vec::new();
    let xs: [f64; 2] = [-0.5, 0.5];
    let ys: [f64; 2] = [-0.5, 0.5];
    let zs: [f64; 2] = [-0.5, 0.5];

    // Create cube (8 points in space)
    for &x in &xs {
        for &y in &ys {
            for &z in &zs {
                points.push(rotation::Point{ x, y, z });
            }
        }
    }

    let edges: [(usize, usize); 12] = [(0,1), (0,2), (0,4), (1,3), (1,5), (2,3), (2,6), (3,7), (4,5), (4,6), (5,7), (6,7)];

    // Initial tilt
    for p in &mut points {
        p.rotate_rpy(0.0, 0.0, -25.0);
    }

    // Hide the cursor
    print!("\x1B[?25l");

    // Time duration later used to slow down frame generation set to bout 24 fps
    let time: time::Duration = time::Duration::from_millis(41);

    // Spin the cube and display it on the screen
    loop {
        for _ in 1..=365 {
            let mut screen: Vec<Vec<char>> = vec![vec![' '; display::WIDTH as usize]; display::HEIGHT as usize];
            
            for p in &mut points {
                p.rotate_y(1.0);
            }

            for (p1, p2) in edges {
                display::prepare_line(&points[p1], &points[p2], &mut screen);
            }

            let _ = stdout().flush();
            display::render(&screen);
            thread::sleep(time);
        }
    }
}