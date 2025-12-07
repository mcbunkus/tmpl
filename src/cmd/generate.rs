/// gen is a reserved keyword, that's why this module doesn't match the other's naming convention.
use anyhow::{Context, Result};
use minijinja::Environment;
use std::{
    env::set_current_dir,
    fs::{create_dir_all, write},
    io::Write,
};
use toml::value::Datetime;

use crate::cli::GenArgs;

use crate::{
    io::IO,
    path::check_path_is_valid,
    specs::{Spec, Specs},
};

/// Merge options specified by the user through the command line, with variables defined in their
/// spec. The command line option is added to this map if it doesn't already exist, otherwise, it
/// overwrites the variable defined in the spec. This gives the user the ability to define defaults
/// in the spec, but easily override them when generating the spec from the command line.
fn merge_options(defaults: &toml::Table, options: Vec<String>) -> toml::Table {
    let mut variables = defaults.clone();

    let chunks = options.chunks(2).filter_map(|chunk| {
        if chunk.len() == 2 {
            Some((chunk[0].clone(), chunk[1].clone()))
        } else {
            None
        }
    });

    for (key, var) in chunks {
        let value = var
            .parse::<i64>()
            .map(toml::Value::Integer)
            .or_else(|_| var.parse::<f64>().map(toml::Value::Float))
            .or_else(|_| var.parse::<bool>().map(toml::Value::Boolean))
            .or_else(|_| var.parse::<Datetime>().map(toml::Value::Datetime))
            .unwrap_or_else(|_| toml::Value::String(var.clone()));

        variables.insert(key, value);
    }

    variables
}

/// generate corresponds to the gen subcommand. It generates the given template spec
pub fn generate<Stdout: Write, Stderr: Write>(
    specs: &Specs,
    args: GenArgs,
    io: &mut IO<Stdout, Stderr>,
) -> Result<()> {
    if let Some(path) = args.workdir {
        set_current_dir(path).context("Unable to change the current working directory")?;
    }

    let spec: Spec = specs
        .read_spec(&args.name)
        .context("Unable to parse template file")?;

    // Merging options specified by the user with the defaults in their spec.
    let variables = merge_options(&spec.variables, args.options);

    // minijinja
    let mut env = Environment::new();

    let mut errors = Vec::new();

    for t in &spec.templates {
        check_path_is_valid(&t.path)?;

        let result = (|| -> Result<()> {
            let name = t
                .path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in template path"))?;

            env.add_template(name, &t.body)?;
            let render = env.get_template(name)?.render(&variables)?;

            if let Some(parent) = t.path.parent() {
                create_dir_all(parent)?;
            }

            write(&t.path, render)?;
            writeln!(io.stdout(), "{}", name)
                .context("Failed to write name of path to stdout writer")?;
            Ok(())
        })();

        if let Err(e) = result {
            errors.push((t.path.display().to_string(), e));
        }
    }

    if !errors.is_empty() {
        writeln!(
            io.stderr(),
            "\nThe following errors occurred while generating {}",
            args.name.display()
        )
        .context("Failed to write preamble to error to stderr writer")?;

        for (path, e) in &errors {
            writeln!(io.stderr(), "\t{}: {:#}", path, e)
                .context("Failed to write paths that had errors to stderr writer")?;
        }

        return Err(anyhow::anyhow!(
            "{} template(s) in {} failed to generate",
            errors.len(),
            args.name.display()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_options_empty_options() {
        let mut defaults = toml::Table::new();
        defaults.insert("user".to_string(), toml::Value::String("alice".to_string()));
        defaults.insert("count".to_string(), toml::Value::Integer(5));

        let result = merge_options(&defaults, vec![]);

        assert_eq!(result, defaults);
    }

    #[test]
    fn test_merge_options_string_values() {
        let defaults = toml::Table::new();
        let options = vec!["name".to_string(), "Bob".to_string()];

        let result = merge_options(&defaults, options);

        assert_eq!(
            result.get("name"),
            Some(&toml::Value::String("Bob".to_string()))
        );
    }

    #[test]
    fn test_merge_options_integer_parsing() {
        let defaults = toml::Table::new();
        let options = vec!["count".to_string(), "42".to_string()];

        let result = merge_options(&defaults, options);

        assert_eq!(result.get("count"), Some(&toml::Value::Integer(42)));
    }

    #[test]
    fn test_merge_options_boolean_parsing() {
        let defaults = toml::Table::new();
        let options = vec![
            "enabled".to_string(),
            "true".to_string(),
            "disabled".to_string(),
            "false".to_string(),
        ];

        let result = merge_options(&defaults, options);

        assert_eq!(result.get("enabled"), Some(&toml::Value::Boolean(true)));
        assert_eq!(result.get("disabled"), Some(&toml::Value::Boolean(false)));
    }

    #[test]
    fn test_merge_options_overrides_defaults() {
        let mut defaults = toml::Table::new();
        defaults.insert("user".to_string(), toml::Value::String("alice".to_string()));
        defaults.insert("count".to_string(), toml::Value::Integer(5));

        let options = vec!["user".to_string(), "bob".to_string()];

        let result = merge_options(&defaults, options);

        assert_eq!(
            result.get("user"),
            Some(&toml::Value::String("bob".to_string()))
        );
        assert_eq!(result.get("count"), Some(&toml::Value::Integer(5))); // unchanged
    }

    #[test]
    fn test_merge_options_odd_number_ignored() {
        let mut defaults = toml::Table::new();
        defaults.insert(
            "existing".to_string(),
            toml::Value::String("value".to_string()),
        );

        // 3 options - the last one has no value so should be ignored
        let options = vec![
            "key1".to_string(),
            "value1".to_string(),
            "incomplete".to_string(),
        ];

        let result = merge_options(&defaults, options);

        assert_eq!(
            result.get("key1"),
            Some(&toml::Value::String("value1".to_string()))
        );
        assert_eq!(result.get("incomplete"), None); // not added
        assert_eq!(
            result.get("existing"),
            Some(&toml::Value::String("value".to_string()))
        ); // unchanged
    }

    #[test]
    fn test_merge_options_invalid_toml_becomes_string() {
        let defaults = toml::Table::new();
        let options = vec!["weird".to_string(), "not-valid-toml!@#".to_string()];

        let result = merge_options(&defaults, options);

        assert_eq!(
            result.get("weird"),
            Some(&toml::Value::String("not-valid-toml!@#".to_string()))
        );
    }
}
