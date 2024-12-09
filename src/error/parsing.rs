use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

pub struct ParsingError {
    code: usize,
    message: String,
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        Ok(())
    }
}

impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Parsing Error {:03}] {}", self.code, self.message)?;
        Ok(())
    }
}

impl ParsingError {
    pub fn unexpected_token(token: &str) -> Self {
        let message = format!("Unexpected token while parsing, got: {}", token);
        Self { code: 1, message }
    }

    pub fn definition_not_found() -> Self {
        let message = "Found problem clause before problem definition";
        Self {
            code: 2,
            message: message.to_string(),
        }
    }

    pub fn mul_definition() -> Self {
        let message = "Found more than one problem definition";
        Self {
            code: 3,
            message: message.to_string(),
        }
    }
}
