pub enum Header {
    Subject(String),
    ContentType(String),
    From(String),
    To(String),
    ReturnPath(String),
    MessageId(String),
}

pub struct EmailMessage {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub headers: Vec<Header>,
    pub body: MessageBody,
}

pub struct MessageBody {
    pub data: String,
}
