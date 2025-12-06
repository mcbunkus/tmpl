use std::io::{self, Write};

/// IO encapsulates a stdout and stderr writer. It exists to facilitate integration testing.
/// For normal use, the normal stdout and stderr writers are used, but a Vec<u8> is used in tests
/// instead for testing output.
pub struct IO<Stdout = io::Stdout, Stderr = io::Stderr> {
    stdout: Stdout,
    stderr: Stderr,
}

impl<Stdout: Write, Stderr: Write> IO<Stdout, Stderr> {
    pub fn new(stdout: Stdout, stderr: Stderr) -> Self {
        IO { stdout, stderr }
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    pub fn stderr(&mut self) -> &mut Stderr {
        &mut self.stderr
    }
}

impl Default for IO {
    fn default() -> Self {
        Self {
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }
}
