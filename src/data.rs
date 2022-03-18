//! Handles WebGl data.
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{Error, Result};

pub enum WebGlDataKind {
    Attribute,
    Varying,
}

/// A WebGL attribute.
pub struct WebGlData {
    id: &'static str,
    location: u32,
    kind: WebGlDataKind,
}

impl WebGlData {
    fn get_location_i32(id: &str, ctx: &WebGl2RenderingContext, prog: &WebGlProgram) -> i32 {
        ctx.get_attrib_location(prog, id)
    }

    fn new(
        id: &'static str,
        kind: WebGlDataKind,
        ctx: &WebGl2RenderingContext,
        prog: &WebGlProgram,
    ) -> Result<Self> {
        use WebGlDataKind::{Attribute, Varying};

        let location_i32 = match kind {
            Attribute => ctx.get_attrib_location(prog, id),
            Varying => ctx.get_,
        };

        let location = location_i32
            .try_into()
            .map_err(|_| Error::NoDataLocation(id.into()))?;

        Ok(Self { id, location, kind })
    }
}
