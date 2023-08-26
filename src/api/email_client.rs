use async_trait::async_trait;

use super::api_email_dto::GmailApiEmailMessage;
use super::errors::ApiError;

#[async_trait]
trait ApiClient {
    async fn authenticate_user() -> Result<String, ApiError>;
    async fn get_emails() -> Result<Vec<GmailApiEmailMessage>, ApiError>;
    async fn send_email(email: GmailApiEmailMessage) -> Result<String, ApiError>;
    async fn delete_email(email: GmailApiEmailMessage) -> Result<String, ApiError>;
    async fn archive_email(email: GmailApiEmailMessage) -> Result<String, ApiError>;
}
