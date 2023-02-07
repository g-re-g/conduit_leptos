use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Field<T> {
    pub input: Option<T>,
    pub errors: Vec<FieldError>,
}

impl<T> Field<T> {
    pub fn required(input: Option<T>) -> Self {
        let errors = match input {
            Some(_) => vec![],
            None => vec![FieldError::Required],
        };
        Field { input, errors }
    }
}

impl Field<String> {
    pub fn min_length(mut self, min: usize) -> Self {
        if let Some(s) = &self.input {
            if s.len() < min {
                self.errors.push(FieldError::MinLength(min));
            }
        }
        self
    }

    pub fn email(mut self) -> Self {
        if let Some(s) = &self.input {
            // TODO: obviously this needs to be more robust
            if s.split('@').collect::<Vec<_>>().len() != 2 {
                self.errors.push(FieldError::InvalidEmail)
            }
        }
        self
    }

    pub fn trim(mut self) -> Self {
        if let Some(s) = &self.input {
            self.input = Some(s.trim().to_string())
        }
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldError {
    Required,
    MinLength(usize),
    InvalidEmail,
}

impl leptos::IntoView for FieldError {
    fn into_view(self, cx: leptos::Scope) -> leptos::View {
        self.to_string().into_view(cx)
    }
}

impl fmt::Display for FieldError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            FieldError::Required => "This field is required.".to_string(),
            FieldError::MinLength(min) => {
                format!("This field must be at least {} characters.", min)
            }
            FieldError::InvalidEmail => {
                "This field doesn't look like an email address.".to_string()
            }
        };
        write!(f, "{}", msg)
    }
}
