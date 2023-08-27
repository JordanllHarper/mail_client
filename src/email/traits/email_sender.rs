use super::email_message::EmailMessage;

#[async_trait]
pub trait EmailSender {
    fn send_email(
        &self,
        email: EmailMessage,
        domain: String,
        user_credentials: UserCredentials,
    ) -> Result<(), EmailError>;
}
