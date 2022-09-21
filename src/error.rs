use std::error::Error;
use std::fmt::{Display, Formatter};

/// 表达式解析错误
/// # 字段
/// - `message` 错误描述
#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        ParseError {
            message: msg.to_string()
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}
