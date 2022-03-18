use reedcg::{App, Sphere};
use wasm_bindgen::JsValue;

fn main() -> Result<(), JsValue> {
    Sphere::default().run()
}
