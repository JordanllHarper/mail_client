use crate::email::email_message::MessageBody;

//NOTE: This may expand in the future to parse attachment data
pub fn parse_body(m: String) -> MessageBody {
    MessageBody { data: m }
}
