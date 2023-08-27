use async_trait::async_trait;

use crate::email::{email_client::GetEmailRequest, email_message::EmailMessage};

#[async_trait]
pub trait EmailReceiver {
    fn get_emails(&self, email_request: GetEmailRequest)
        -> eyre::Result<Option<Vec<EmailMessage>>>;
}
