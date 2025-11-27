use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Component, Path, PathBuf},
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

    /// Ensures the given spec name doesn't contain any path shenanigans. This is similar to
    /// path_is_safe is path.rs but tweaked for spec names specifically
    fn validate_spec_name(&self, name: &OsStr) -> Result<()> {
        let path = Path::new(name);

        if path.file_name() != Some(name) {
            bail!("{} must be a simple filename", name.display());
        }

        if name == "." || name == ".." {
            bail!("{} is not a valid spec name", name.display());
        }

        Ok(())
    }

    /// Return the Specs spec directory.
    pub fn dir(&self) -> PathBuf {
        self.dir.clone()
    }

    pub fn exists(&self, name: &OsStr) -> bool {
        if self.validate_spec_name(name).is_err() {
            return false;
        }

        let path = self.dir.join(name);
        path.is_file()
    }

    /// Read a spec file in the specs directory to a string.
    pub fn read_to_string(&self, name: &OsStr) -> Result<String> {
        self.validate_spec_name(name)
            .context("Unable to read spec to string")?;

        let path = self.dir.join(name);
        fs::read_to_string(path).context(format!("Failed to read {} to string", name.display()))
    }

    /// Get the full path to a spec file, which will return an error if it doesn't exist
    pub fn safe_get_spec_path(&self, name: &OsStr) -> Result<PathBuf> {
        self.validate_spec_name(name)
            .context("Unable to safely get the full path to the spec")?;

        let path = self.dir.join(name);
        ensure!(path.exists(), "{} doesn't exist", name.display());
        ensure!(path.is_file(), "{} is not a file", name.display());
        Ok(path)
    }

    /// Deserialize a spec file and return it as a Spec struct.
    pub fn read_spec(&self, name: &OsStr) -> Result<Spec> {
        self.validate_spec_name(name)
            .context("Unable to read spec")?;

        let contents = self
            .read_to_string(name)
            .context("Unable to open spec file for reading")?;

        toml::from_str(&contents).context("Unable to parse template file")
    }

    /// Delete a spec file.
    pub fn delete_spec(&self, name: &OsStr) -> Result<()> {
        self.validate_spec_name(name)
            .context("Unable to delete spec")?;

        let path = self.safe_get_spec_path(name)?;
        fs::remove_file(path).context("Failed to delete spec")
    }

    /// Write a Spec struct to a file in the spec directory.
    pub fn write_spec(&self, name: &OsStr, spec: &Spec) -> Result<()> {
        self.validate_spec_name(name)
            .context("Unable to write spec")?;

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

    pub fn copy(&self, src_name: &OsStr, dst_name: &OsStr) -> Result<()> {
        self.validate_spec_name(src_name)
            .context(format!("Failed to copy {}", src_name.display()))?;

        self.validate_spec_name(dst_name).context(format!(
            "Failed to copy {} to {}",
            src_name.display(),
            dst_name.display()
        ))?;

        let src = self.dir.join(src_name);
        let dst = self.dir.join(dst_name);

        ensure!(src.exists(), "{} doesn't exist, can't copy", src.display());

        ensure!(
            src != dst,
            "Cannot copy a spec to itself, this would result in truncation"
        );

        fs::copy(src, dst)?;
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
