#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RedirectToWebSocket {
    pub WebSocketPort: u16,
    pub ProtocolVersion: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTextInComponent<'a> {
    pub text: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeInText {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTextFromComponent {
    pub selections: Vec<RangeInText>,
    pub syntax: String,
    pub text: String,
    pub title: String,
    pub url: String,
}
