use async_trait::async_trait;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use super::{
    email_message::EmailMessage,
    errors::EmailError,
    user_credentials::{self, UserCredentials},
};
pub fn parse_attr(attr: &String) -> Result<Mailbox, EmailError> {
    let attr_from: Mailbox = match attr.parse() {
        Ok(v) => v,
        Err(_) => return Err(EmailError::ParseError("Error parsing".to_string())),
    };
    Ok(attr_from)
}

fn handle_content_type(f: &super::email_message::ContentType) -> ContentType {
    match f {
        super::email_message::ContentType::TextPlain => ContentType::TEXT_PLAIN,
        super::email_message::ContentType::TextHtml => ContentType::TEXT_HTML,
    }
}
//Email client to handle email operations
#[async_trait]
pub trait EmailClient {
    fn get_emails(&self) -> Result<Vec<EmailMessage>, EmailError>;
    fn send_email(
        &self,
        email: EmailMessage,
        domain: String,
        user_credentials: UserCredentials,
    ) -> Result<(), EmailError>;
    fn delete_email(&self, email: EmailMessage) -> Result<(), EmailError>;
    fn archive_email(&self, email: EmailMessage) -> Result<(), EmailError>;
}

pub struct EmailClientLettreImpl {}

impl EmailClient for EmailClientLettreImpl {
    fn get_emails(&self) -> Result<Vec<EmailMessage>, EmailError> {
        todo!()
    }

    fn send_email(
        &self,
        email: EmailMessage,
        domain: String,
        credentials: UserCredentials,
    ) -> Result<(), EmailError> {
        let m = Message::builder()
            .from(parse_attr(&email.from)?)
            .to(parse_attr(&email.to)?)
            .subject(email.subject)
            .header(handle_content_type(&email.content_type))
            .body(email.body.data)
            .unwrap();

        let mailer = match SmtpTransport::relay(&domain) {
            Ok(r) => r,
            Err(e) => return Err(EmailError::TransportError { e }),
        }
        .credentials(credentials.to_lettre_creds())
        .build();

        return match mailer.send(&m) {
            Ok(_) => Ok(()),
            Err(_) => Err(EmailError::SendError),
        };
    }

    fn delete_email(&self, email: EmailMessage) -> Result<(), EmailError> {
        todo!()
    }

    fn archive_email(&self, email: EmailMessage) -> Result<(), EmailError> {
        todo!()
    }
}
