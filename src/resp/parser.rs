use crate::enums::sign::Sign;
use crate::errors::app_errors::AppError;

pub enum Parser {
    SimpleString(String),
    SimpleError(String),
    BulkString(String),
    NullBulkString,
    Array(Vec<String>),
    Integer(Option<Sign>, u16),
}

impl Parser {
    pub fn serialize(&self) -> String {
        match self {
            Parser::SimpleString(s) => format!("+{}\r\n", s),
            Parser::SimpleError(s) => format!("-{}\r\n", s),
            Parser::NullBulkString => "$-1\r\n".to_string(),
            Parser::Array(v) => self.get_array_string(v),
            Parser::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Parser::Integer(s, i) => self.get_integer_string(s, *i),
        }
    }

    fn get_array_string(&self, v: &Vec<String>) -> String {
        let mut array_string = format!("*{}\r\n", v.len());
        for s in v {
            array_string.push_str(&format!("${}\r\n{}\r\n", s.len(), s));
        }
        array_string
    }

    fn get_integer_string(&self, sign: &Option<Sign>, i: u16) -> String {
        match sign {
            Some(sign) => format!(":{:?}{}\r\n", sign, i),
            None => format!(":{}\r\n", i),
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


