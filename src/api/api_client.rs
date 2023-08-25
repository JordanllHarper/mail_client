//TODO: Change to results for success and failure

use async_trait::async_trait;

use super::api_email_dto::ApiEmailMessage;
use super::errors::ApiError;

#[async_trait]
trait ApiClient {
    async fn authenticate_user() -> Result<String, ApiError>;
    async fn get_emails() -> Result<String, ApiError>;
    async fn send_email(email: ApiEmailMessage) -> Result<String, ApiError>;
    async fn delete_email(email: ApiEmailMessage) -> Result<String, ApiError>;
    async fn archive_email(email: ApiEmailMessage) -> Result<String, ApiError>;
}
