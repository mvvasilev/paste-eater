use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PasteEaterError {
    internal_error: Option<Box<dyn Error>>,
    message: String
}

impl PasteEaterError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            internal_error: None
        }
    }

    pub fn new_internal(message: &str, internal: Box<dyn Error>) -> Self {
        Self {
            message: message.to_string(),
            internal_error: Some(internal)
        }
    }
}

impl Display for PasteEaterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "Paste eater error: {}, internal: {}", 
            self.message, 
            self.internal_error.as_ref().map_or("".to_string(), |e| format!("{}", e))
        )
    }
}

impl Error for PasteEaterError {
}