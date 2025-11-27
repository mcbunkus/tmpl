use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

/// Spec defines a full user template spec. It includes all the variables the user is setting (and
/// their defaults), and all the files tmpl will generate.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Spec {
    pub variables: toml::Table,
    pub templates: Vec<Template>,
}

/// Template defines an entry in the spec, that contains the contents of a file, and its path. The
/// path can be nested arbitrarily deep, tmpl will create parent directories as necessarry.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Template {
    pub path: PathBuf,
    pub body: String,
}

/// Specs represents a collection of specs, co-located in a directory. It provides programmatic
/// access to spec files.
pub struct Specs {
    dir: PathBuf,
}

impl Specs {
    /// Create a new Specs struct, returns an Error if the given spec directory doesn't exist or
    /// it's not a directory.
    pub fn new(dir: &Path) -> Result<Self> {
        ensure!(dir.exists(), "{} doesn't exist", dir.display());
        ensure!(dir.is_dir(), "{} is not a directory", dir.display());

        Ok(Self {
            dir: dir.to_path_buf(),
        })
    }

    /// Return the Specs spec directory.
    pub fn dir(&self) -> PathBuf {
        self.dir.clone()
    }

    /// Read a spec file in the specs directory to a string.
    pub fn read_to_string(&self, name: &OsStr) -> Result<String> {
        let path = self.dir.join(name);
        fs::read_to_string(path).context(format!("Failed to read {} to string", name.display()))
    }

    /// Get the full path to a spec file.
    pub fn get_spec_path(&self, name: &OsStr) -> Result<PathBuf> {
        let path = self.dir.join(name);
        ensure!(path.exists(), "{} doesn't exist", name.display());
        ensure!(path.is_file(), "{} is not a file", name.display());
        Ok(path)
    }

    /// Deserialize a spec file and return it as a Spec struct.
    pub fn read_spec(&self, name: &OsStr) -> Result<Spec> {
        let contents = self
            .read_to_string(name)
            .context("Unable to open spec file for reading")?;

        toml::from_str(&contents).context("Unable to parse template file")
    }

    /// Delete a spec file.
    pub fn delete_spec(&self, name: &OsStr) -> Result<()> {
        let path = self.get_spec_path(name)?;
        fs::remove_file(path).context("Failed to delete spec")
    }

    /// Write a Spec struct to a file in the spec directory.
    pub fn write_spec(&self, name: &OsStr, spec: &Spec) -> Result<()> {
        // doing this manually cause we care if it *does* exist
        let path = self.dir.join(name);
        ensure!(!path.exists(), "{} already exists", name.display());

        let toml_string =
            toml::to_string(&spec).context("Failed to convert spec to rendered TOML")?;

        fs::write(&path, toml_string).context(format!(
            "Failed to write rendered TOML to {}",
            path.display()
        ))?;

        Ok(())
    }

    /// Get all of the specs in the spec directory
    pub fn get_all_specs(&self) -> Result<Vec<OsString>> {
        let entries = fs::read_dir(&self.dir)?
            .filter_map(|r| {
                if let Ok(e) = r {
                    if e.path().is_file() {
                        Some(e.file_name())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(entries)
    }
}
