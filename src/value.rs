impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(number) => number.to_string(),
            Value::String(string) => string.to_string(),
            Value::Nothing => "Nothing".to_string(),
        }
    }
}

/// Represents a value.
#[derive(Clone)]
pub enum Value {
    Number(i32),
    String(String),
    Nothing,
}
