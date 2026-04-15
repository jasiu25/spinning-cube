pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    // Basically a debug function
    #[allow(dead_code)]
    pub fn show(&self) {
        println!("({}, {}, {})", self.x, self.y, self.z);
    }

    pub fn rotate_y (&mut self, angle: f64) {
        let angle: f64 = angle.to_radians(); // Conversion for using sin and cos methods
        let x: f64 = self.x * angle.cos() + self.z * angle.sin();
        let z: f64 = -1.0 * self.x * angle.sin() + self.z * angle.cos();

        self.x = x;
        self.z = z;
    }

    pub fn rotate_rpy (&mut self, alpha: f64, beta: f64, gamma: f64) {
        let alpha: f64 = alpha.to_radians();
        let beta: f64 = beta.to_radians();
        let gamma: f64 = gamma.to_radians();
        let (ca, sa) = (alpha.cos(), alpha.sin());
        let (cb, sb) = (beta.cos(), beta.sin());
        let (cg, sg) = (gamma.cos(), gamma.sin());

        let x: f64 = self.z * (ca * cb) + self.y * (ca * sb * sg - sa * cg) + self.x * (ca * sb * cg + sa * sg);
        let y: f64 = self.z * (sa * cb) + self.y * (sa * sb * sg + ca * cg) + self.x * (sa * sb * cg - ca * sg);
        let z: f64 = self.z * -sb + self.y * (cb * sg) + self.x * (cb * cg);
        
        self.x = x;
        self.y = y;
        self.z = z;
    }
}