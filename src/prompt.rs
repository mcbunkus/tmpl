use std::io::{self, BufRead, Write};

use anyhow::Result;

pub fn prompt_yn(question: &str, default_answer: bool) -> Result<bool> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    prompt_yn_with_io(
        question,
        default_answer,
        &mut stdin.lock(),
        &mut stdout.lock(),
    )
}

fn prompt_yn_with_io<R: BufRead, W: Write>(
    question: &str,
    default_answer: bool,
    reader: &mut R,
    writer: &mut W,
) -> Result<bool> {
    let default_hint = if default_answer { "Y/n" } else { "y/N" };

    loop {
        write!(writer, "{} ({}): ", question, default_hint)?;
        writer.flush()?;

        let mut input = String::new();
        reader.read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            "" => return Ok(default_answer),
            _ => println!("Please answer 'y' or 'n'"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn y() {
        let input = b"y\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let result = prompt_yn_with_io("Test?", false, &mut reader, &mut writer).unwrap();
        assert!(result, "Expected true but got false");
    }

    #[test]
    fn n() {
        let input = b"n\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let result = prompt_yn_with_io("Test?", true, &mut reader, &mut writer).unwrap();
        assert!(!result, "Expected false but got true");
    }

    #[test]
    fn yes() {
        let input = b"yes\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let result = prompt_yn_with_io("Test?", false, &mut reader, &mut writer).unwrap();
        assert!(result, "Expected true but got false");
    }

    #[test]
    fn no() {
        let input = b"no\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let result = prompt_yn_with_io("Test?", true, &mut reader, &mut writer).unwrap();
        assert!(!result, "Expected false but got true");
    }
}
