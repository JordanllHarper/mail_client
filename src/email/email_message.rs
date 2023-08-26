pub enum Header {
    ContentType(ContentType),
}

pub enum ContentType {
    TextPlain,
    TextHtml,
}

pub struct EmailMessage {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub content_type: ContentType,
    pub body: MessageBody,
}

pub struct MessageBody {
    pub data: String,
}
