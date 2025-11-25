use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub templates: Vec<Template>,
    pub variables: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub path: String,
    pub body: String,
}
