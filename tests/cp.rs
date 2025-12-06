mod common;
use std::ffi::OsString;

use tmpl::{CpArgs, NewArgs, cmd};

#[test]
fn cp_command() {
    let src_name = OsString::from("test.src");
    let dst_name = OsString::from("test.dst");

    let (temp, specs) = common::create_test_workspace();

    let mut io = common::test_io();

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
    cmd::new(&specs, new_args, &mut io).unwrap();
    cmd::cp(&specs, cp_args).unwrap();

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
