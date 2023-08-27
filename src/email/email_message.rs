pub struct Addresses {
    pub from: String,
    pub to: String,
}

pub struct Headers {
    pub headers: String,
    pub subject: String,
    pub content_type: ContentType,
}
pub enum ContentType {
    TextPlain,
    TextHtml,
}

pub struct EmailMessage {
    pub addresses: Addresses,
    pub headers: Headers,
    pub body: MessageBody,
}

pub struct MessageBody {
    pub data: String,
}
