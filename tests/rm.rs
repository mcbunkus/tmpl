mod common;

use tmpl::{
    cli::{NewArgs, RmArgs},
    cmd,
};

use crate::common::TestWorkspace;
use std::ffi::OsString;

/// Test basic rm functionality
#[test]
fn rm_single_spec() {
    let spec_name = OsString::from("test.spec");
    let mut workspace = TestWorkspace::new();
    let spec_path = workspace.dir.path().join(&spec_name);

    let new_args = NewArgs {
        name: spec_name.clone(),
        edit: false,
    };

    let mut scratch = TestWorkspace::scratch_io();
    cmd::new(&workspace.specs, new_args, &mut scratch).unwrap();

    assert!(
        spec_path.exists(),
        "{} doesn't exist like it should",
        spec_path.display()
    );

    let rm_args = RmArgs {
        to_delete: vec![spec_name.clone()],
        skip_prompt: true,
    };

    cmd::rm(&workspace.specs, rm_args, &mut workspace.io).unwrap();

    assert!(
        !spec_path.exists(),
        "{} exists when it shouldn't",
        spec_path.display()
    );

    let raw_output = workspace.io.stdout().clone();
    let output = String::from_utf8(raw_output).unwrap();
    let expected = format!("Deleted {}", spec_name.display());
    assert_eq!(output.trim(), expected.trim());
}

/// Test rm multi-spec functionality
#[test]
fn rm_multiple_specs() {
    let spec_names = vec![
        OsString::from("test1.spec"),
        OsString::from("test2.spec"),
        OsString::from("test3.spec"),
    ];

    let mut workspace = TestWorkspace::new();

    for spec in spec_names.iter() {
        let spec_path = workspace.dir.path().join(spec);
        let new_args = NewArgs {
            name: spec.clone(),
            edit: false,
        };

        let mut scratch = TestWorkspace::scratch_io();
        cmd::new(&workspace.specs, new_args, &mut scratch).unwrap();
        assert!(
            spec_path.exists(),
            "{} doesn't exist like it should",
            spec_path.display()
        );
    }

    let rm_args = RmArgs {
        to_delete: spec_names.clone(),
        skip_prompt: true,
    };

    cmd::rm(&workspace.specs, rm_args, &mut workspace.io).unwrap();

    for spec in spec_names.iter() {
        let spec_path = workspace.dir.path().join(spec);
        assert!(
            !spec_path.exists(),
            "{} exists when it shouldn't",
            spec_path.display()
        );
    }

    let raw_output = workspace.io.stdout().clone();
    let output = String::from_utf8(raw_output).unwrap();

    for spec in spec_names.iter() {
        let expected_to_contain = format!("Deleted {}", spec.display());
        assert!(
            output.contains(&expected_to_contain),
            "{} does not contain \"{}\"",
            output,
            expected_to_contain
        );
    }
}
