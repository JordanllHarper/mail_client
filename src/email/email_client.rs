use async_trait::async_trait;
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};

use super::{email_message::EmailMessage, errors::EmailError, user_credentials::UserCredentials};
pub fn parse_attr(attr: &String) -> Result<Mailbox, EmailError> {
    let attr_from: Mailbox = match attr.parse() {
        Ok(v) => v,
        Err(_) => return Err(EmailError::ParseError(format!("Error parsing {attr}"))),
    };
    Ok(attr_from)
}

fn handle_content_type(
    f: &super::email_message::ContentType,
) -> lettre::message::header::ContentType {
    match f {
        super::email_message::ContentType::TextPlain => {
            lettre::message::header::ContentType::TEXT_PLAIN
        }
        super::email_message::ContentType::TextHtml => {
            lettre::message::header::ContentType::TEXT_HTML
        }
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
        let m = match Message::builder()
            .from(parse_attr(&email.from)?)
            .to(parse_attr(&email.to)?)
            .subject(email.subject)
            .header(handle_content_type(&email.content_type))
            .body(email.body.data)
        {
            Ok(m) => m,
            Err(_) => return Err(EmailError::AttributeError),
        };

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
