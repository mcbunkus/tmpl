/// Tests the generate subcommand. These have to be done serially, because they rely on changing
/// the current working directory in each test. That's a problem if both of them are running in
/// parallel.
use std::ffi::OsString;
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

#[test]
#[serial]
fn generate() {
    let spec_name = OsString::from("test.spec");
    let mut workspace = TestWorkspace::new();

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

    workspace.specs.write_spec(&spec_name, &spec).unwrap();

    let gen_args = GenArgs {
        name: spec_name.clone(),
        options: vec![],
        workdir: Some(workspace.dir.path().into()),
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

    workspace.specs.write_spec(&spec_name, &spec).unwrap();

    let gen_args = GenArgs {
        name: spec_name.clone(),
        options: vec!["name".into(), "bill".into()],
        workdir: Some(workspace.dir.path().into()),
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
