struct EmailMessage {
    pub id: String,
    pub thread_id: String,
    pub label_ids: Vec<String>,
    pub snippet: String,
    pub history_id: String,
    pub internal_date: String,
    pub payload: EmailMessagePayload,
    pub size_est: i32,
    pub raw: String,
}

struct EmailMessagePayload {
    pub part_id: String,
    pub mime_type: String,
    pub filename: String,
    pub headers: Vec<Header>,
    pub body: MessagePartBody,
    pub sub_payloads: Vec<EmailMessagePayload>,
}

pub struct Header {
    pub name: String,
    pub value: String,
}
pub struct MessagePartBody {
    pub attachment_id: String,
    pub size: i32,
    pub data: String,
}
