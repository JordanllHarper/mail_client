use eyre::Result;
use lettre::{Message, SmtpTransport, Transport};

extern crate imap;
extern crate native_tls;

use super::{
    email_message::EmailMessage,
    errors::EmailError,
    traits::email_sender::EmailSender,
    user_credentials::UserCredentials,
    util::{handle_content_type, parse_attr},
};

pub struct GetEmailRequest {
    domain: String,
    user_credentials: UserCredentials,
    mailbox: String,
    lower_bound: i32,
    upper_bound: i32,
}

pub struct EmailClient {}

impl super::traits::email_receiver::EmailReceiver for EmailClient {
    fn get_emails(&self, email_request: GetEmailRequest) -> Result<Option<Vec<EmailMessage>>> {
        let tls = native_tls::TlsConnector::builder()
            .build()
            .expect("This builder wasn't build correctly!");

        let client = imap::connect(
            (email_request.domain.clone(), 993),
            &email_request.domain,
            &tls,
        )?;

        let mut imap_session = client
            .login(
                email_request.user_credentials.email,
                email_request.user_credentials.password,
            )
            .map_err(|e| e.0)?;

        imap_session.select(email_request.mailbox)?;
        let email_fetch_string = crate::email::util::gen_csv_from_lower(
            email_request.lower_bound,
            email_request.upper_bound,
        );
        let messages = imap_session.fetch(email_fetch_string, "ALL")?;
        
        messages.iter().map(|f| f.header())

        //TODO: Implement parsing headers and bodies and creating email message from that

        todo!();
    }
}

impl EmailSender for EmailClient {
    fn send_email(
        &self,
        email: EmailMessage,
        domain: String,
        credentials: UserCredentials,
    ) -> Result<(), EmailError> {
        let m = if let Ok(m) = Message::builder()
            .from(parse_attr(&email.addresses.from)?)
            .to(parse_attr(&email.addresses.to)?)
            .subject(email.headers.subject)
            .header(handle_content_type(&email.headers.content_type))
            .body(email.body.data)
        {
            m
        } else {
            return Err(EmailError::AttributeError);
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
}
