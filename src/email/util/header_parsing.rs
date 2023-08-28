use crate::email::{
    email_message::{Addresses, ContentType, Headers},
    errors::EmailError,
};
use mailparse::*;

//Parses the content type and associates that with the represented enum
//NOTE: This may expand in the future
pub fn parse_content_type(content_type_string: String) -> Option<ContentType> {
    match content_type_string.as_str() {
        "text/plain" => Some(ContentType::TextPlain),
        "text/html" => Some(ContentType::TextHtml),
        _ => return None,
    }
}

//Parses the data out of the header
//NOTE: This may expand in the future
pub fn parse_headers(m: &[u8]) -> eyre::Result<crate::email::email_message::Headers> {
    let result = mailparse::parse_mail(m)?;

    let from = (&result.headers)
        .get_first_value("From")
        .ok_or(crate::email::errors::EmailError::ParseHeaderError)?;
    let to = result
        .headers
        .get_first_value("To")
        .ok_or(EmailError::ParseHeaderError)?;
    let subject = result
        .headers
        .get_first_value("Subject")
        .ok_or(EmailError::ParseHeaderError)?;
    let content_type = result
        .headers
        .get_first_value("Content-Type")
        .ok_or(EmailError::ParseHeaderError)?;

    Ok(Headers {
        addresses: Addresses { from, to },
        subject,
        content_type: parse_content_type(content_type).ok_or(EmailError::ParseHeaderError)?,
    })
}
