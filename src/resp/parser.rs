use crate::enums::sign::Sign;
use crate::errors::app_errors::AppError;

pub enum Parser {
    SimpleString(String),
    SimpleError(String),
    BulkString(String),
    NullBulkString,
    Integer(Option<Sign>, u16),
}

impl Parser {
    pub fn serialize(self) -> String {
        match self {
            Parser::SimpleString(s) => format!("+{}\r\n", s),
            Parser::SimpleError(s) => format!("-{}\r\n", s),
            Parser::NullBulkString => "$-1\r\n".to_string(),
            Parser::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Parser::Integer(s, i) => match s {
                Some(sign) => format!(":{:?}{}\r\n", sign, i),
                None => format!(":{}\r\n", i),
            },
        }
    }
}

pub async fn extract_set_command_args(args: Vec<String>) -> Result<(String, String, usize), AppError> {
    if args.len() < 2 {
        return Err(AppError::WrongNumberOfArgumentsError);
    }

    let key = args[0].clone();
    let value = args[1].clone();
    let expiration = if args.len() > 3 && args[2] == "px" {
        match args[3].parse() {
            Ok(exp) => exp,
            Err(_) => {
                return Err(AppError::InvalidExpirationValue);
            }
        }
    } else {
        0
    };

    Ok((key, value, expiration))
}


