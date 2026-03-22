struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn show(&self) {
        println!("({}, {}, {})", self.x, self.y, self.z);
    }

    fn rotate_x (&mut self, angle: f64) {
        let radians: f64 = angle.to_radians();
        let x: f64 = self.x;
        let y: f64 = self.y * radians.cos() - self.z * radians.sin();
        let z: f64 = self.y * radians.sin() + self.z * radians.cos();
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn rotate_y (&mut self, angle: f64) {
        let radians: f64 = angle.to_radians();
        let x: f64 = self.x * radians.cos() + self.z * radians.sin();
        let y: f64 = self.y;
        let z: f64 = -1.0 * self.x * radians.sin() + self.z * radians.cos();
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn rotate_z (&mut self, angle: f64) {
        let radians: f64 = angle.to_radians();
        let x: f64 = self.x * radians.cos() - self.y * radians.sin();
        let y: f64 = self.x * radians.sin() + self.y * radians.cos();
        let z: f64 = self.z;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

fn main() {
    let p1: Point = Point {
        x: 0.5,
        y: 0.7,
        z: 0.5,
    };
    let p2: Point = Point {
        x: 0.5,
        y: -0.7,
        z: 0.5,
    };
    let p3: Point = Point {
        x: 0.5,
        y: 0.7,
        z: -0.5,
    };
    let p4: Point = Point {
        x: 0.5,
        y: -0.7,
        z: -0.5,
    };
    let p5: Point = Point {
        x: -0.5,
        y: 0.7,
        z: 0.5,
    };
    let p6: Point = Point {
        x: -0.5,
        y: -0.7,
        z: 0.5,
    };
    let p7: Point = Point {
        x: -0.5,
        y: 0.7,
        z: -0.5,
    };
    let p8: Point = Point {
        x: -0.5,
        y: -0.7,
        z: -0.5,
    };

    let points: [Point; 8] = [p1, p2, p3, p4, p5, p6, p7, p8];
    for mut p in points {
        p.rotate_x(15.0);
        p.rotate_y(15.0);
        p.rotate_z(15.0);
        p.show();
    }
}
