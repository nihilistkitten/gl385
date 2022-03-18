//! Contains the custom error type.
use crate::ShaderKind;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An invalid shader was passed.
    #[error("{1} shader failed to compile: {0}")]
    InvalidShader(String, ShaderKind),

    /// An error occured creating the shader program.
    #[error("failed to create shader program: {0}")]
    ShaderProgramCreation(String),

    /// Buffer creation failed.
    #[error("buffer creation failed")]
    BufferCreation,

    /// No data location found.
    #[error("no data location found for {0}")]
    NoDataLocation(String),
}
