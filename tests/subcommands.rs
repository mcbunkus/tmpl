use std::ffi::OsString;

use tempfile::tempdir;
use tmpl::{
    cmd::{self, new},
    specs::Specs,
};

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

#[test]
fn test_cp_command() {
    let src_name = OsString::from("test.src");
    let dst_name = OsString::from("test.dst");

    let temp = tempdir().unwrap();
    let specs = Specs::new(temp.path()).unwrap();

    // this will succeed if test_new_command is succeeding
    new(&specs, &src_name, false).unwrap();
    cmd::cp(&specs, &src_name, &dst_name, true).unwrap();

    let dst_path = temp.path().join(&dst_name);
    assert!(dst_path.exists(), "{} does not exist", dst_path.display());

    let src_content = specs.read_to_string(&src_name).unwrap();
    let dst_content = specs.read_to_string(&dst_name).unwrap();

    assert_eq!(
        src_content,
        dst_content,
        "expected {} and {} to match",
        src_name.display(),
        dst_name.display()
    );
}
