use async_trait::async_trait;
use lettre::{
    message::{header::ContentType, MessageBuilder},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use super::{email_message::EmailMessage, errors::EmailError};

#[async_trait]
pub trait EmailClient {
    fn get_emails(&self) -> Result<Vec<EmailMessage>, EmailError>;
    fn send_email(&self, email: EmailMessage) -> Result<(), EmailError>;
    fn delete_email(&self, email: EmailMessage) -> Result<(), EmailError>;
    fn archive_email(&self, email: EmailMessage) -> Result<(), EmailError>;
}

pub struct EmailClientImpl {
    builder: MessageBuilder,
    creds: Credentials,
    mailer: SmtpTransport,
}

impl EmailClient for EmailClientImpl {
    fn get_emails(&self) -> Result<Vec<EmailMessage>, EmailError> {
        todo!()
    }

    fn send_email(&self, email: EmailMessage) -> Result<(), EmailError> {
        todo!()
    }

    fn delete_email(&self, email: EmailMessage) -> Result<(), EmailError> {
        todo!()
    }

    fn archive_email(&self, email: EmailMessage) -> Result<(), EmailError> {
        todo!()
    }
}
