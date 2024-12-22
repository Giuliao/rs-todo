use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn new(input_title: &str) -> Self {
        match input_title {
            "DONE" => Self::DONE,
            "PENDING" => Self::PENDING,
            _ => panic!("input '{}' not supported", input_title),
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            Self::DONE => "DONE".to_string(),
            Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(input: String) -> Self {
        match input.as_str() {
            "DONE" => Self::DONE,
            "PENDING" => Self::PENDING,
            _ => panic!("input {} not supported", input),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DONE => write!(f, "DONE"),
            Self::PENDING => write!(f, "PENDING"),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}
