use std::fmt::Display;

use astro_utils::error::AstroUtilError;

#[derive(Debug, Clone)]
pub(crate) enum ElenathError {
    AstroError(String),
}

impl Display for ElenathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElenathError::AstroError(err) => write!(f, "{}", err),
        }
    }
}

impl From<AstroUtilError> for ElenathError {
    fn from(v: AstroUtilError) -> Self {
        Self::AstroError(v.to_string())
    }
}
