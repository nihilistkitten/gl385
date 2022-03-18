mod app;
mod data;
mod error;
mod js;
mod load_shader;
mod revolution;
mod sphere;
mod torus;

use error::{Error, Result};
use js::{bind_buffer, fill_buffer, resize_canvas};
use load_shader::{make_shader_program, ShaderKind};

pub use app::App;
pub use revolution::Revolution;
pub use sphere::Sphere;
pub use torus::Torus;
