mod rotation;

fn main() {
    let mut points: Vec<rotation::Point> = Vec::new();
    let xs = [-0.5, 0.5];
    let ys = [-0.5, 0.5];
    let zs = [-0.5, 0.5];

    // Create cube (8 points in space)
    for &x in &xs {
        for &y in &ys {
            for &z in &zs {
                points.push(rotation::Point{ x, y, z });
            }
        }
    }

    // Initial tilt
    for p in &mut points {
        p.rotate_rpy(45.0, 25.0, 15.0);
        p.show();
    }

    for angle in 0..=5 {
        for p in &mut points {
            spin(p, angle as f64);
        }
    }
}

fn spin(p: &mut rotation::Point, angle: f64) {
    p.rotate_z(angle);
    p.show();
}