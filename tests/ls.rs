mod common;

use std::{ffi::OsString, vec};

use tmpl::{
    cli::LsArgs,
    cmd::{self},
    specs::{Spec, Template},
};

use crate::common::TestWorkspace;

fn create_test_spec() -> Spec {
    let mut spec = Spec {
        variables: toml::map::Map::new(),
        templates: vec![],
    };

    spec.variables.insert("name".into(), "world".into());

    spec.templates.push(Template {
        path: "README.md".into(),
        body: "Hello, {{ name }}".into(),
    });

    spec
}

#[test]
fn ls() {
    let mut workspace = TestWorkspace::new();

    let sources = vec![
        OsString::from("test1"),
        OsString::from("test2"),
        OsString::from("test3"),
    ];

    let test_spec = create_test_spec();

    for source in &sources {
        workspace.specs.write_spec(source, &test_spec).unwrap();
    }

    let args = LsArgs { list_vars: false };
    cmd::list(&workspace.specs, args, &mut workspace.io).unwrap();

    let output = String::from_utf8_lossy(workspace.io.stdout());

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

#[test]
fn ls_with_vars() {
    let mut workspace = TestWorkspace::new();

    let sources = vec![
        OsString::from("test1"),
        OsString::from("test2"),
        OsString::from("test3"),
    ];

    let test_spec = create_test_spec();

    for source in &sources {
        workspace.specs.write_spec(source, &test_spec).unwrap();
    }

    let args = LsArgs { list_vars: true };
    cmd::list(&workspace.specs, args, &mut workspace.io).unwrap();

    let output = String::from_utf8_lossy(workspace.io.stdout());
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
        assert!(
            line.contains("name=\"world\""),
            "{} doesn't contain expected option output",
            line
        );
    }
}
