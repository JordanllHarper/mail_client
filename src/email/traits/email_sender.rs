use async_trait::async_trait;

use crate::email::{
    email_message::EmailMessage, errors::EmailError, user_credentials::UserCredentials,
};

#[async_trait]
pub trait EmailSender {
    fn send_email(
        &self,
        email: EmailMessage,
        domain: String,
        user_credentials: UserCredentials,
    ) -> Result<(), EmailError>;
}
