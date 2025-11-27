use std::ffi::OsString;

use tempfile::tempdir;
use tmpl::{cmd::new, specs::Specs};

#[test]
fn test_new_command() {
    let spec_name = OsString::from("test.template");

    let temp = tempdir().unwrap();
    let specs = Specs::new(temp.path()).unwrap();

    new(&specs, &spec_name, false).unwrap();

    let full_path = temp.path().join(&spec_name);
    assert!(
        full_path.exists(),
        "expected {} to exist",
        full_path.display()
    );

    let parsed_spec = specs.read_spec(&spec_name).unwrap();
    let default_spec = tmpl::cmd::new::default_spec();

    assert_eq!(parsed_spec, default_spec);
}
