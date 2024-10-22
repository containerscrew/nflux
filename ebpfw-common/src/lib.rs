#![no_std]

#[derive(Debug)]
pub enum Action {
    Dropped,
    Accepted,
    Unknown, // Para manejar posibles valores no esperados
}

impl From<i32> for Action {
    fn from(value: i32) -> Self {
        match value {
            2 => Action::Dropped,
            3 => Action::Accepted,
            _ => Action::Unknown,
        }
    }
}

impl Action {
    pub fn to_str(&self) -> &'static str {
        match self {
            Action::Dropped => "dropped",
            Action::Accepted => "accepted",
            Action::Unknown => "unknown",
        }
    }
}
