use crate::App;

const PI: f32 = std::f32::consts::PI;

pub struct Sphere {
    smoothness: u32,
    radius: f32,
    accessed: bool,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            smoothness: 24,
            radius: 1.0,
            accessed: false,
        }
    }
}

impl App for Sphere {
    fn update(&mut self) -> bool {
        true
    }

    fn render(&self) -> Vec<(f32, f32, f32)> {
        let mut out = vec![];
        for i in 0..self.smoothness {
            let theta = self.normalize_tau(i);
            let theta_next = self.normalize_tau(i + 1);

            for j in 0..self.smoothness {
                let phi = self.normalize_pi(j);
                let phi_next = self.normalize_pi(j + 1);

                let p0 = self.polar_projection(theta, phi);
                let p1 = self.polar_projection(theta, phi_next);
                let p2 = self.polar_projection(theta_next, phi);
                let p3 = self.polar_projection(theta_next, phi_next);

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

impl Sphere {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn smoothness(self, smoothness: u32) -> Self {
        Self { smoothness, ..self }
    }

    #[must_use]
    pub fn radius(self, radius: impl Into<f32>) -> Self {
        Self {
            radius: radius.into(),
            ..self
        }
    }

    /// The vertex in three-space corersponding to the given polar coordinates on the sphere.
    fn polar_projection(&self, theta: f32, phi: f32) -> (f32, f32, f32) {
        let x = self.radius * theta.cos() * phi.sin();
        let y = self.radius * theta.sin() * phi.sin();
        let z = self.radius * phi.cos();

        (x, y, z)
    }

    /// Given a parametrized i, return the corresponding theta in radians.
    fn normalize_tau(&self, i: u32) -> f32 {
        i as f32 / self.smoothness as f32 * 2.0 * PI
    }

    /// Given a parametrized i, return the corresponding phi in radians.
    fn normalize_pi(&self, j: u32) -> f32 {
        j as f32 / self.smoothness as f32 * PI
    }
}
