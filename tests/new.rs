mod common;
use std::ffi::OsString;

use tmpl::{cli::NewArgs, cmd};

use crate::common::TestWorkspace;

#[test]
fn new() {
    let spec_name = OsString::from("test.template");
    let mut workspace = TestWorkspace::new();

    let args = NewArgs {
        name: spec_name.clone(),
        edit: false,
    };

    cmd::new(&workspace.specs, args, &mut workspace.io).unwrap();

    let full_path = workspace.dir.path().join(&spec_name);
    assert!(
        full_path.exists(),
        "expected {} to exist",
        full_path.display()
    );

    let parsed_spec = workspace.specs.read_spec(&spec_name).unwrap();
    let default_spec = tmpl::cmd::new::default_spec();

    assert_eq!(parsed_spec, default_spec);
}
