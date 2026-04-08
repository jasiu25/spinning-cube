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
    let (x0, y0) = to_screen(project(p1));
    let (x1, y1) = to_screen(project(p2));

    if (y1 - y0).abs() < (x1 - x0).abs() {
        prepare_horizontal_line(x0, y0, x1, y1, screen);
    } else {
        prepare_vertical_line(x0, y0, x1, y1, screen);
    }
}

#[allow(dead_code)]
fn prepare_horizontal_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, screen: &mut Vec<Vec<char>>) {
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx: i32 = x1 - x0;
    let mut dy: i32 = y1 - y0;

    // Set line's direction depending on delta y
    let dir: i32 = if dy < 0 { -1 } else { 1 };
    dy *= dir;

    let mut d: i32 = 2*dy - dx;
    let mut y: i32 = y0;

    for x in x0 as i32..=x1 as i32 {
        prepare_point((x as f64, y as f64), screen);
        if d > 0 {
            y += dir;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }
}

#[allow(dead_code)]
fn prepare_vertical_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, screen: &mut Vec<Vec<char>>) {
    if y0 > y1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }
    let mut dx: i32 = x1 - x0;
    let dy: i32 = y1 - y0;

    // Set line's direction depending on delta y
    let dir: i32 = if dy < 0 { -1 } else { 1 };
    dx *= dir;

    let mut d: i32 = 2*dx - dy;
    let mut x: i32 = x0;

    for y in y0 as i32..=y1 as i32 {
        prepare_point((x as f64, y as f64), screen);
        if d > 0 {
            x += dir;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }
}




