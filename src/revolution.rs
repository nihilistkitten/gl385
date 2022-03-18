use crate::App;

const PI: f32 = std::f32::consts::PI;

pub struct Revolution {
    smoothness: u32,
    curve: Vec<(f32, f32)>,
    accessed: bool,
}

impl App for Revolution {
    fn update(&mut self) -> bool {
        if self.accessed {
            false
        } else {
            self.accessed = true;
            true
        }
    }

    fn render(&self) -> Vec<(f32, f32, f32)> {
        let mut out = vec![];
        for i in 0..self.smoothness {
            let theta_curr = self.normalize_tau(i);
            let theta_next = self.normalize_tau(i + 1);
            for p in 0..self.curve.len() - 1 {
                let lower = self.curve[p];
                let upper = self.curve[p + 1];

                let p0 = rotate(lower, theta_curr);
                let p1 = rotate(lower, theta_next);
                let p2 = rotate(upper, theta_curr);
                let p3 = rotate(upper, theta_next);

                out.push(p0);
                out.push(p2);
                out.push(p1);

                out.push(p3);
                out.push(p1);
                out.push(p2);
            }
        }

        log::error!("{:?}", &out);

        out
    }
}

impl Revolution {
    #[must_use]
    pub fn new(curve: Vec<(f32, f32)>) -> Self {
        Self {
            smoothness: 24,
            accessed: false,
            curve,
        }
    }

    #[must_use]
    /// Produce the points of a curve from a function, evaluated samples times on [-1, 1].
    pub fn from_func(f: impl Fn(f32) -> (f32, f32), samples: i32) -> Self {
        let lim = samples / 2;
        let curve = (-lim..lim)
            .map(|i| f(2.0 * i as f32 / samples as f32))
            .collect();
        Self::new(curve)
    }

    #[must_use]
    pub fn smoothness(self, smoothness: u32) -> Self {
        Self { smoothness, ..self }
    }

    /// Given a parametrized i, return the corresponding theta in radians.
    fn normalize_tau(&self, i: u32) -> f32 {
        i as f32 / self.smoothness as f32 * 2.0 * PI
    }
}

/// Given a point in 2-space, rotate it theta radians around the y axis.
fn rotate((px, py): (f32, f32), theta: f32) -> (f32, f32, f32) {
    (px, py * theta.cos(), py * theta.sin())
}
