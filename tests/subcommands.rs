use std::ffi::OsString;

use tempfile::tempdir;
use tmpl::{
    cmd::{self, new},
    specs::Specs,
};

#[test]
fn test_new_command() {
    let mut stdout = Vec::new();

    let spec_name = OsString::from("test.template");

    let temp = tempdir().unwrap();
    let specs = Specs::new(temp.path()).unwrap();

    new(&specs, &spec_name, false, &mut stdout).unwrap();

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
    let mut stdout = Vec::new();
    let src_name = OsString::from("test.src");
    let dst_name = OsString::from("test.dst");

    let temp = tempdir().unwrap();
    let specs = Specs::new(temp.path()).unwrap();

    // this will succeed if test_new_command is succeeding
    new(&specs, &src_name, false, &mut stdout).unwrap();
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

#[test]
fn test_ls_command() {
    let mut stdout = Vec::new();
    let sources = vec![
        OsString::from("test1"),
        OsString::from("test2"),
        OsString::from("test3"),
    ];

    let temp = tempdir().unwrap();
    let specs = Specs::new(temp.path()).unwrap();

    for source in &sources {
        cmd::new(&specs, source, false, &mut stdout).unwrap();
    }

    let mut output_buffer = Vec::new();
    cmd::list(&specs, false, &mut output_buffer).unwrap();
    let output = String::from_utf8_lossy(&output_buffer);

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
}
