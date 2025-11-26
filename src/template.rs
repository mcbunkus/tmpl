use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

/// Spec defines a full user template spec. It includes all the variables the user is setting (and
/// their defaults), and all the files tmpl will generate.
#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub variables: HashMap<String, toml::Value>,
    pub templates: Vec<Template>,
}

/// Template defines an entry in the spec, that contains the contents of a file, and its path. The
/// path can be nested arbitrarily deep, tmpl will create parent directories as necessarry.
#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub path: PathBuf,
    pub body: String,
}
