use astro_utils::error::AstroUtilError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum ElenathError {
    AstroError(String),
    IoError(String),
    NoCelestialSystem,
}

impl Display for ElenathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElenathError::AstroError(err) => write!(f, "{}", err),
            ElenathError::IoError(err) => write!(f, "{}", err),
            ElenathError::NoCelestialSystem => write!(f, "No celestial system loaded."),
        }
    }
}

impl From<AstroUtilError> for ElenathError {
    fn from(v: AstroUtilError) -> Self {
        Self::AstroError(v.to_string())
    }
}

impl From<std::io::Error> for ElenathError {
    fn from(v: std::io::Error) -> Self {
        Self::IoError(v.to_string())
    }
}
