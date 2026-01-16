use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub fn hytale(path: impl Into<String>) -> Self {
        Self::new("hytale", path)
    }

    pub fn parse(s: &str) -> Option<Self> {
        let (namespace, path) = s.split_once(':')?;
        Some(Self::new(namespace, path))
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self::parse(s).unwrap_or_else(|| Self::hytale(s))
    }
}
