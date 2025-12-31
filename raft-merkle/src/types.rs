use std::fmt;
use std::io::Cursor;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Append { entry: String },
}

impl Request {
    pub fn append(entry: impl Into<String>) -> Self {
        Request::Append {
            entry: entry.into(),
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::Append { entry } => write!(f, "Append{{ entry : {} }}", entry),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub index: Option<u64>,
}

impl Response {
    pub fn new(index: impl Into<u64>) -> Self {
        Response {
            index: Some(index.into()),
        }
    }

    pub fn none() -> Self {
        Response { index: None }
    }
}

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
);
