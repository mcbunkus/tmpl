mod common;

use std::{ffi::OsString, vec};

use tmpl::{
    cli::{LsArgs, NewArgs},
    cmd::{self},
};

#[test]
fn ls_command() {
    let mut new_io = common::test_io();

    let sources = vec![
        OsString::from("test1"),
        OsString::from("test2"),
        OsString::from("test3"),
    ];

    // "_temp" and not "_" because "_" runs TempDir's destructor and delete the test directory before the
    // test is done
    let (_temp, specs) = common::create_test_workspace();

    for source in &sources {
        let args = NewArgs {
            name: source.clone(),
            edit: false,
        };
        cmd::new(&specs, args, &mut new_io).unwrap();
    }

    let args = LsArgs { list_vars: false };
    let mut list_io = common::test_io();
    cmd::list(&specs, args, &mut list_io).unwrap();

    let output = String::from_utf8_lossy(list_io.stdout());

    for source in &sources {
        let source_str = source.to_string_lossy();
        assert!(
            output.contains(source_str.as_ref()),
            "expected output to contain '{}', got:\n{}",
            source_str,
            output
        );
    }

    let line_count = output.lines().count();
    assert_eq!(line_count, sources.len());

    for line in output.lines() {
        let as_ostring = OsString::from(line);
        assert!(sources.contains(&as_ostring));
    }
}
