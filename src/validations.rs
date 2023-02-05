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

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldError {
    Required,
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
            FieldError::Required => "This field is required.",
        };
        write!(f, "{}", msg)
    }
}
