use lettre::{SmtpTransport, Transport};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Connection timed out")]
    ConnectionTimeout,
    #[error("Client error")]
    ClientError { e: lettre::error::Error },
    #[error("Client error")]
    TransportError {
        e: <SmtpTransport as Transport>::Error,
    },
    #[error("Parse error")]
    ParseError(String),
    #[error("Parse error")]
    SendError,

    #[error("Attribute error")]
    AttributeError,
}
