use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn ls(template_directory: PathBuf) -> Result<()> {
    let mut count = 0;
    for entry in fs::read_dir(&template_directory)? {
        let path = entry?.path();

        if path.is_file()
            && let Some(name) = path.file_name()
        {
            println!("{}", name.display());
            count += 1;
        }
    }

    if count == 0 {
        println!(
            "You don't have any templates yet, create a one with \"tmpl new <template name>\"."
        );
    }

    Ok(())
}
