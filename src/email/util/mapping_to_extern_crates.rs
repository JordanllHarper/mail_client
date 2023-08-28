use lettre::message::Mailbox;

pub fn parse_attr(attr: &String) -> Result<Mailbox, crate::email::errors::EmailError> {
    let attr_from: Mailbox = match attr.parse() {
        Ok(v) => v,
        Err(_) => {
            return Err(crate::email::errors::EmailError::ParseError(format!(
                "Error parsing {attr}"
            )))
        }
    };
    Ok(attr_from)
}

pub fn handle_content_type(
    f: crate::email::email_message::ContentType,
) -> lettre::message::header::ContentType {
    match f {
        crate::email::email_message::ContentType::TextPlain => {
            lettre::message::header::ContentType::TEXT_PLAIN
        }
        crate::email::email_message::ContentType::TextHtml => {
            lettre::message::header::ContentType::TEXT_HTML
        }
    }
}
