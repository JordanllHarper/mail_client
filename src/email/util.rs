use lettre::message::Mailbox;

use super::{
    email_message::{EmailMessage, Headers},
    errors::EmailError,
};

pub fn parse_attr(attr: &String) -> Result<Mailbox, EmailError> {
    let attr_from: Mailbox = match attr.parse() {
        Ok(v) => v,
        Err(_) => return Err(EmailError::ParseError(format!("Error parsing {attr}"))),
    };
    Ok(attr_from)
}

pub fn handle_content_type(
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

pub fn gen_csv_from_lower(lower_bound: i32, upper_bound: i32) -> String {
    let mut returned_string = "".to_string();

    for i in lower_bound..upper_bound {
        returned_string = returned_string + &i.to_string();
    }
    returned_string
}

pub fn parse_headers() -> Headers {}

pub fn parse_body() {}
