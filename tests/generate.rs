/// Tests the generate subcommand. These have to be done serially, because they rely on changing
/// the current working directory in each test. That's a problem if both of them are running in
/// parallel.
use std::ffi::OsString;
use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;

use serial_test::serial;
use tmpl::cli::GenArgs;
use tmpl::cmd;
use tmpl::specs::Spec;
use tmpl::specs::Template;

use crate::common::TestWorkspace;

mod common;

const TEMPLATE_PATH: &str = "README.md";

const TEMPLATE_BODY: &str = "\
# Template
Hello, {{ name }}
";

fn create_test_spec() -> Spec {
    let mut spec = Spec {
        variables: toml::map::Map::new(),
        templates: Vec::new(),
    };

    let template = Template {
        path: PathBuf::from(TEMPLATE_PATH),
        body: String::from(TEMPLATE_BODY),
    };

    spec.templates.push(template);

    spec.variables
        .insert("name".into(), toml::value::Value::String("testing".into()));

    spec
}

#[test]
#[serial]
fn generate_with_spec_name() {
    let spec_name = OsString::from("test.spec");
    let mut workspace = TestWorkspace::new();

    let spec = create_test_spec();
    workspace.specs.write_spec(&spec_name, &spec).unwrap();

    let gen_args = GenArgs {
        name: spec_name.clone().into(),
        options: vec![],
        workdir: Some(workspace.dir.path().into()),
        spec_file: None,
    };

    cmd::generate(&workspace.specs, gen_args, &mut workspace.io).unwrap();

    let raw_file_contents = read_to_string(TEMPLATE_PATH).unwrap();
    let file_contents = raw_file_contents.trim();
    let expected = "\
# Template
Hello, testing
"
    .trim();

    assert_eq!(file_contents, expected);
}

#[test]
#[serial]
fn generate_with_spec_file() {
    let mut workspace = TestWorkspace::new();
    let spec_file = workspace.dir.path().join("spec.toml");

    let spec = create_test_spec();

    let serialized_spec = toml::to_string(&spec).unwrap();
    fs::write(&spec_file, serialized_spec).unwrap();

    let gen_args = GenArgs {
        name: None,
        options: vec![],
        workdir: Some(workspace.dir.path().into()),
        spec_file: Some(spec_file),
    };

    cmd::generate(&workspace.specs, gen_args, &mut workspace.io).unwrap();

    let raw_file_contents = read_to_string(TEMPLATE_PATH).unwrap();
    let file_contents = raw_file_contents.trim();
    let expected = "\
# Template
Hello, testing
"
    .trim();

    assert_eq!(file_contents, expected);
}

#[test]
#[serial]
fn generate_with_options() {
    let spec_name = OsString::from("test.spec");
    let mut workspace = TestWorkspace::new();

    let spec = create_test_spec();
    workspace.specs.write_spec(&spec_name, &spec).unwrap();

    let gen_args = GenArgs {
        name: spec_name.clone().into(),
        options: vec!["name".into(), "bill".into()],
        workdir: Some(workspace.dir.path().into()),
        spec_file: None,
    };

    cmd::generate(&workspace.specs, gen_args, &mut workspace.io).unwrap();

    let raw_file_contents = read_to_string(TEMPLATE_PATH).unwrap();
    let file_contents = raw_file_contents.trim();
    let expected = "\
# Template
Hello, bill
"
    .trim();

    assert_eq!(file_contents, expected);
}

#[test]
#[serial]
fn neither_name_nor_file_returns_error() {
    let mut workspace = TestWorkspace::new();
    let gen_args = GenArgs {
        name: None,
        options: vec![],
        workdir: None,
        spec_file: None,
    };
    assert!(cmd::generate(&workspace.specs, gen_args, &mut workspace.io).is_err());
}

#[test]
#[serial]
fn both_name_and_file_returns_error() {
    let mut workspace = TestWorkspace::new();
    let gen_args = GenArgs {
        name: Some("some.name".into()),
        options: vec![],
        workdir: None,
        spec_file: Some("some/path.toml".into()),
    };
    assert!(cmd::generate(&workspace.specs, gen_args, &mut workspace.io).is_err());
}
