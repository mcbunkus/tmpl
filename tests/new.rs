mod common;
use std::ffi::OsString;

use tmpl::{cli::NewArgs, cmd};

#[test]
fn new_command() {
    let spec_name = OsString::from("test.template");

    let (temp, specs) = common::create_test_workspace();

    let mut io = common::test_io();

    let args = NewArgs {
        name: spec_name.clone(),
        edit: false,
    };

    cmd::new(&specs, args, &mut io).unwrap();

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
