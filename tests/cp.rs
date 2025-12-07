mod common;
use std::ffi::OsString;

use tmpl::cli::{CpArgs, NewArgs};

use tmpl::cmd;

use crate::common::TestWorkspace;

#[test]
fn cp() {
    let src_name = OsString::from("test.src");
    let dst_name = OsString::from("test.dst");

    let mut workspace = TestWorkspace::new();

    let new_args = NewArgs {
        name: src_name.clone(),
        edit: false,
    };

    let cp_args = CpArgs {
        source: src_name.clone(),
        dest: dst_name.clone(),
        skip_prompt: true,
    };

    // this will succeed if test_new_command is succeeding
    cmd::new(&workspace.specs, new_args, &mut workspace.io).unwrap();
    cmd::cp(&workspace.specs, cp_args).unwrap();

    let dst_path = workspace.dir.path().join(&dst_name);
    assert!(dst_path.exists(), "{} does not exist", dst_path.display());

    let src_content = workspace.specs.read_to_string(&src_name).unwrap();
    let dst_content = workspace.specs.read_to_string(&dst_name).unwrap();

    assert_eq!(
        src_content,
        dst_content,
        "expected {} and {} to match",
        src_name.display(),
        dst_name.display()
    );
}

#[test]
fn cp_overwrite_results_in_error() {
    let name = OsString::from("test.name");
    let workspace = TestWorkspace::new();
    let result = workspace.specs.copy(&name, &name);
    assert!(result.is_err());
}
