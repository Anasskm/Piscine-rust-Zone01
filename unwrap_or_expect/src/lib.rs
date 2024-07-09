pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            server.expect("Unknown security level panic");
            unreachable!()
        }
        Security::High => {
            server.expect("ERROR: program stops");
            unreachable!()
        }
        Security::Medium => {
            server.unwrap_or("WARNING: check the server".to_string())
        }
        Security::Low => {
            server.unwrap_or_else(|err| format!("Not found: {}", err))
        }
        Security::BlockServer => {
            server.unwrap_err()
        }
    }
}
