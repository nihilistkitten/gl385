use gl385::{App, Revolution};
use wasm_bindgen::JsValue;

/* struct SphereApp {
    n: f32,
}

impl App for SphereApp {
    fn update(&mut self) -> bool {
        true
    }

    fn render(&self) -> Vec<(f32, f32, f32)> {
        vec![
            (-self.n, -self.n, 0.0),
            (self.n, -self.n, 0.0),
            (0.0, self.n, 0.0),
        ]
    }
} */

fn curve(i: f32) -> (f32, f32) {
    (i, i * i)
}

fn main() -> Result<(), JsValue> {
    Revolution::from_func(curve, 100).smoothness(100).run()
}
