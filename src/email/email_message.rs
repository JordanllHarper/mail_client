#[derive(Clone)]
pub struct Addresses {
    pub from: String,
    pub to: String,
}

#[derive(Clone)]
pub struct Headers {
    pub addresses: Addresses,
    pub subject: String,
    pub content_type: ContentType,
}

#[derive(Clone, Copy)]
pub enum ContentType {
    TextPlain,
    TextHtml,
}
#[derive(Clone)]
pub struct EmailMessage {
    pub headers: Headers,
    pub body: MessageBody,
}
#[derive(Clone)]
pub struct MessageBody {
    pub data: String,
}
