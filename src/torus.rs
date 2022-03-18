use crate::App;

const PI: f32 = std::f32::consts::PI;

pub struct Torus {
    smoothness: u32,
    major_radius: f32,
    minor_radius: f32,
    accessed: bool,
}

impl Default for Torus {
    fn default() -> Self {
        Self {
            smoothness: 24,
            major_radius: 0.7,
            minor_radius: 0.3,
            accessed: false,
        }
    }
}

impl App for Torus {
    fn update(&mut self) -> bool {
        true
    }

    fn render(&self) -> Vec<(f32, f32, f32)> {
        let mut out = vec![];
        for i in 0..self.smoothness {
            let major_curr = self.normalize_tau(i);
            let major_next = self.normalize_tau(i + 1);

            for j in 0..self.smoothness {
                let minor_curr = self.normalize_tau(j);
                let minor_next = self.normalize_tau(j + 1);

                let p0 = self.torus_projection(major_curr, minor_curr);
                let p1 = self.torus_projection(major_curr, minor_next);
                let p2 = self.torus_projection(major_next, minor_curr);
                let p3 = self.torus_projection(major_next, minor_next);

                out.push(p0);
                out.push(p2);
                out.push(p1);

                out.push(p3);
                out.push(p1);
                out.push(p2);
            }
        }
        out
    }
}

impl Torus {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn smoothness(self, smoothness: u32) -> Self {
        Self { smoothness, ..self }
    }

    #[must_use]
    pub fn major_radius(self, major_radius: f32) -> Self {
        Self {
            major_radius,
            ..self
        }
    }

    #[must_use]
    pub fn minor_radius(self, minor_radius: f32) -> Self {
        Self {
            minor_radius,
            ..self
        }
    }

    /// The vertex in three-space corersponding to the given toroidal coordinates on the torus.
    fn torus_projection(&self, major: f32, minor: f32) -> (f32, f32, f32) {
        let xy_radius = self.minor_radius.mul_add(minor.cos(), self.major_radius);

        let x = xy_radius * major.cos();
        let y = xy_radius * major.sin();
        let z = xy_radius * minor.sin();

        (x, y, z)
    }

    /// Given a parametrized i, return the corresponding theta in radians.
    fn normalize_tau(&self, i: u32) -> f32 {
        i as f32 / self.smoothness as f32 * 2.0 * PI
    }
}
