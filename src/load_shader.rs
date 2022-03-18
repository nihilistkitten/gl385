use crate::{Error, Result};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[derive(Clone, Copy, Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

impl std::fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Vertex => write!(f, "vertex"),
            ShaderKind::Fragment => write!(f, "fragment"),
        }
    }
}

impl From<ShaderKind> for u32 {
    fn from(sk: ShaderKind) -> Self {
        match sk {
            ShaderKind::Vertex => WebGl2RenderingContext::VERTEX_SHADER,
            ShaderKind::Fragment => WebGl2RenderingContext::FRAGMENT_SHADER,
        }
    }
}

/// Load a shader of the given kind from the given GLSL source into the given context.
fn load_shader(
    context: &WebGl2RenderingContext,
    source: &'static str,
    kind: ShaderKind,
) -> Result<WebGlShader> {
    let shader = context
        .create_shader(kind.into())
        .expect("web_sys types are valid");

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(Error::InvalidShader(
            context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "unknown error creating shader".into()),
            kind,
        ))
    }
}

/// Link the shaders to the program.
fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram> {
    let program = context.create_program().ok_or_else(|| {
        Error::ShaderProgramCreation("could not create the shader program".into())
    })?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(Error::ShaderProgramCreation(
            context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")),
        ))
    }
}

/// Make the shader program from the given GLSL shader sources.
///
/// # Errors
/// If the shader was invalid, an error is returned.
///
pub fn make_shader_program(
    context: &WebGl2RenderingContext,
    vert_shader: &'static str,
    frag_shader: &'static str,
) -> Result<WebGlProgram> {
    let vert_shader = load_shader(context, vert_shader, ShaderKind::Vertex)?;
    let frag_shader = load_shader(context, frag_shader, ShaderKind::Fragment)?;
    link_program(context, &vert_shader, &frag_shader)
}
