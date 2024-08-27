pub enum Parser {
    SimpleString(String),
    BulkString(String),
    SimpleError(String),
}

impl Parser {
    pub fn serialize(self) -> String {
        match self {
            Parser::SimpleString(s) => format!("+{}\r\n", s),
            Parser::SimpleError(s) => format!("-{}\r\n", s),
            Parser::BulkString(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
        }
    }
}

