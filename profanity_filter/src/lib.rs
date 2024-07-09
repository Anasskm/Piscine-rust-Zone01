pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.contains("stupid") || self.content.is_empty() {
            return None;
        }
        return Some(&self.content);
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(m) => (true, m),
        _ => (false, "ERROR: illegal"),
    }
}