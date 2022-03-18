use reedcg::{App, Sphere};
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

fn main() -> Result<(), JsValue> {
    Sphere::default().run()
}
