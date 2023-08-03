use std::{error::Error, fmt, io};

#[derive(thiserror::Error)]
pub enum ClientError {
    #[error("failed to make the request")]
    RequestError(#[from] io::Error),

    #[error("invalid response")]
    InvalidResponseError(String),

    #[error("unexpected status code")]
    ResponseStatusError(u32),

    #[error("failed to parse data")]
    ParseError(#[from] unhtml::Error),
}

impl fmt::Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
