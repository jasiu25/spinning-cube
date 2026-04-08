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

    pub fn rotate_rpy (&mut self, alpha: f64, beta: f64, gamma: f64) -> &mut Self {
        let alpha: f64 = alpha.to_radians();
        let beta: f64 = beta.to_radians();
        let gamma: f64 = gamma.to_radians();
        let (ca, sa) = (alpha.cos(), alpha.sin());
        let (cb, sb) = (beta.cos(), beta.sin());
        let (cg, sg) = (gamma.cos(), gamma.sin());
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let new_x: f64 = z * (ca * cb) + y * (ca * sb * sg - sa * cg) + x * (ca * sb * cg + sa * sg);
        let new_y: f64 = z * (sa * cb) + y * (sa * sb * sg + ca * cg) + x * (sa * sb * cg - ca * sg);
        let new_z: f64 = z * -sb + y * (cb * sg) + x * (cb * cg);
        
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;

        self
    }
}