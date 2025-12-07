use anyhow::{Context, Result, bail, ensure};
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

#[cfg(test)]
mod tests {
    use std::fs::create_dir;

    use tempfile::tempdir;

    use super::*;

    fn dummy_spec() -> Spec {
        Spec {
            variables: toml::map::Map::new(),
            templates: vec![],
        }
    }

    #[test]
    fn validate_spec_name() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        // ok
        let ok_name = OsString::from("this.is.ok");

        // not ok
        let absolute_path_name = OsString::from("/this/is/not/ok");
        let nested_dir_name = OsString::from("not/ok");
        let cwd_name = OsString::from(".");
        let parent_name = OsString::from("..");
        let parent_dir_name = OsString::from("../not.ok");

        assert!(specs.validate_spec_name(&ok_name).is_ok());
        assert!(specs.validate_spec_name(&absolute_path_name).is_err());
        assert!(specs.validate_spec_name(&nested_dir_name).is_err());
        assert!(specs.validate_spec_name(&cwd_name).is_err());
        assert!(specs.validate_spec_name(&parent_name).is_err());
        assert!(specs.validate_spec_name(&parent_dir_name).is_err());
    }

    #[test]
    fn dir() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();
        assert_eq!(dir.path(), specs.dir);
    }

    #[test]
    fn exists() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let not_exists = OsString::from("does.not.exist");
        assert!(!specs.exists(&not_exists));

        let exists = OsString::from("exists");
        let spec = dummy_spec();

        specs.write_spec(&exists, &spec).unwrap();
        assert!(specs.exists(&exists));
    }

    #[test]
    fn read_to_string() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let spec_name = OsString::from("test.spec");
        let spec = dummy_spec();

        specs.write_spec(&spec_name, &spec).unwrap();

        let content = specs.read_to_string(&spec_name).unwrap();
        assert!(!content.is_empty(), "spec file is empty");
        assert!(
            content.contains("templates"),
            "spec file doesn't contain templates entry"
        );
        assert!(
            content.contains("variables"),
            "spec file doesn't contain variables entry"
        );
    }

    #[test]
    fn safe_get_spec_path() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let spec_name = OsString::from("test.spec");
        let spec = dummy_spec();
        specs.write_spec(&spec_name, &spec).unwrap();

        let full_path = specs.safe_get_spec_path(&spec_name).unwrap();
        assert_eq!(dir.path().join(spec_name), full_path);

        // returns an error if it doesn't exist
        let doesnt_exist = OsString::from("doesnt.exist");
        assert!(specs.safe_get_spec_path(&doesnt_exist).is_err());

        // ensure that safe_get_spec_path returns an error when the name is a directory
        let bad_dir_name = OsString::from("bad_dir");
        let bad_dir = dir.path().join(&bad_dir_name);
        create_dir(&bad_dir).unwrap();
        assert!(specs.safe_get_spec_path(&bad_dir_name).is_err());
    }

    #[test]
    fn read_spec() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let spec_name = OsString::from("test.spec");
        let mut spec = dummy_spec();

        spec.variables
            .insert("option".into(), toml::Value::Integer(42));

        spec.templates.push(Template {
            path: "README.md".into(),
            body: "Hello, world!".into(),
        });

        specs.write_spec(&spec_name, &spec).unwrap();

        let deserialized_spec = specs.read_spec(&spec_name).unwrap();
        assert_eq!(spec, deserialized_spec);
    }

    #[test]
    fn delete_spec() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let spec_name = OsString::from("test.spec");
        let spec = dummy_spec();
        specs.write_spec(&spec_name, &spec).unwrap();

        assert!(dir.path().join(&spec_name).exists());

        specs.delete_spec(&spec_name).unwrap();

        assert!(!dir.path().join(&spec_name).exists());

        assert!(
            specs
                .delete_spec(OsString::from("doesnt.exist").as_os_str())
                .is_err()
        );
    }

    #[test]
    fn write_spec() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let spec_name = OsString::from("test.spec");
        let spec = dummy_spec();

        assert!(specs.write_spec(&spec_name, &spec).is_ok());

        // attempting to do it again should result in an error, other errors are OS dependent
        // (failed to write, etc.)
        assert!(specs.write_spec(&spec_name, &spec).is_err());
    }

    #[test]
    fn copy() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let first_spec = OsString::from("first.spec");
        let second_spec = OsString::from("second.spec");
        let spec_data = dummy_spec();

        specs.write_spec(&first_spec, &spec_data).unwrap();

        assert!(specs.copy(&first_spec, &second_spec).is_ok());

        // attempting to copy to itself should result in an error
        assert!(specs.copy(&first_spec, &first_spec).is_err());

        // can't copy from something that doesn't exist either
        let false_src = OsString::from("false.src");
        let false_dst = OsString::from("false.dst");
        assert!(specs.copy(&false_src, &false_dst).is_err());

        // can't copy from an invalid spec name
        let invalid_src = OsString::from("/not/a/good/spec/name");
        assert!(specs.copy(&invalid_src, &false_dst).is_err());
    }

    #[test]
    fn get_all_specs() {
        let dir = tempdir().unwrap();
        let specs = Specs::new(dir.path()).unwrap();

        let spec_names = [
            OsString::from("spec.1"),
            OsString::from("spec.2"),
            OsString::from("spec.3"),
        ];

        let blank_spec = dummy_spec();

        for spec_name in spec_names.iter() {
            specs.write_spec(spec_name, &blank_spec).unwrap();
        }

        let all_specs = specs.get_all_specs().unwrap();
        assert_eq!(all_specs.len(), spec_names.len());
        for spec_name in all_specs.iter() {
            assert!(spec_names.contains(spec_name));
        }
    }
}
