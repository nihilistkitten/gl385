use gl385::{App, Torus};
use wasm_bindgen::JsValue;

fn curve(i: f32) -> (f32, f32) {
    (i, i * i)
}

fn main() -> Result<(), JsValue> {
    Torus::new().run()
}
