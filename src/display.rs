use crate::rotation::Point;

pub static WIDTH: i32 = 80;
pub static HEIGHT: i32 = 40;
pub static DISTANCE: f64 = 2.0;

pub fn prepare_point((x, y): (f64, f64), screen: &mut Vec<Vec<char>>) {
    if x < WIDTH as f64 && y < HEIGHT as f64 {
        screen[y as usize][x as usize] = '#';
    }
}

pub fn render(screen: &Vec<Vec<char>>) {
    // Move the cursor to the beginning of the screen
    print!("\x1B[H");
    for row in screen {
        println!("{}", row.iter().collect::<String>());
    }
}

fn project(p: &Point) -> (f64, f64) {
    let x: f64 = p.x / (p.z + DISTANCE);
    let y: f64 = p.y / (p.z + DISTANCE);
    
    (x, y)
}

fn to_screen((x, y): (f64, f64)) -> (i32, i32) {
    let sx: i32  = ((x + 1.0) * (WIDTH as f64 / 2.0)) as i32;
    let sy: i32  = ((1.0 - y) * (HEIGHT as f64 / 2.0)) as i32;

    (sx, sy)
}

// Bresenham's line algorithm
pub fn prepare_line(p1: &Point, p2: &Point, screen: &mut Vec<Vec<char>>) {
    let (mut x0, mut y0) = to_screen(project(p1));
    let (x1, y1) = to_screen(project(p2));

    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };

    let dx: i32 = (x1 - x0).abs();
    let dy: i32 = (y1 - y0).abs();

    let mut balance = dx - dy;

    loop {
        prepare_point((x0 as f64, y0 as f64), screen);
        
        if x0 == x1 && y0 == y1 {
            break;
        }

        let b2: i32 = 2 * balance;

        if b2 > -dy {
            balance -= dy;
            x0 += sx;
        }

        if b2 < dx {
            balance += dx;
            y0 += sy;
        }
    }
}



