pub struct Addresses {
    pub from: String,
    pub to: String,
}

pub struct Headers {
    pub addresses: Addresses,
    pub subject: String,
    pub content_type: ContentType,
}
pub enum ContentType {
    TextPlain,
    TextHtml,
}

pub struct EmailMessage {
    pub headers: Headers,
    pub body: MessageBody,
}

pub struct MessageBody {
    pub data: String,
}
