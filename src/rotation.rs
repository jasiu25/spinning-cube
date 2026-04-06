pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn show(&self) {
        println!("({}, {}, {})", self.x, self.y, self.z);
    }

    pub fn rotate_x (&mut self, angle: f64) -> &mut Self {
        let radians: f64 = angle.to_radians();
        let y: f64 = self.y * radians.cos() - self.z * radians.sin();
        let z: f64 = self.y * radians.sin() + self.z * radians.cos();

        self.y = y;
        self.z = z;

        self
    }

    pub fn rotate_y (&mut self, angle: f64) -> &mut Self {
        let radians: f64 = angle.to_radians();
        let x: f64 = self.x * radians.cos() + self.z * radians.sin();
        let z: f64 = -1.0 * self.x * radians.sin() + self.z * radians.cos();

        self.x = x;
        self.z = z;

        self
    }

    pub fn rotate_z (&mut self, angle: f64) -> &mut Self {
        let radians: f64 = angle.to_radians();
        let x: f64 = self.x * radians.cos() - self.y * radians.sin();
        let y: f64 = self.x * radians.sin() + self.y * radians.cos();

        self.x = x;
        self.y = y;

        self
    }

    pub fn rotate_rpy (&mut self, yaw: f64, pitch: f64, roll: f64) -> &mut Self {
        self.rotate_x(roll)
            .rotate_y(pitch)
            .rotate_z(yaw)
    }
}