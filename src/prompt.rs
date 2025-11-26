use std::io::{self, Write};

use anyhow::Result;

pub fn prompt_yn(question: &str, default_answer: bool) -> Result<bool> {
    let default_hint = if default_answer { "Y/n" } else { "y/N" };

    loop {
        print!("{} ({}): ", question, default_hint);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            "" => return Ok(default_answer),
            _ => println!("Please answer 'y' or 'n'"),
        }
    }
}
