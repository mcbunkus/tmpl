use tempfile::TempDir;
use tmpl::{io::IO, specs::Specs};

pub struct TestWorkspace {
    pub io: tmpl::io::IO<Vec<u8>, Vec<u8>>,
    pub specs: Specs,

    // dir is actually being used but the compiler keeps complaining about it never being read,
    // despite the fact that it very much is. Even when "let dir = workspace.dir;". visibility?
    #[allow(dead_code)]
    pub dir: TempDir,
}

impl TestWorkspace {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let specs = tmpl::specs::Specs::new(dir.path()).unwrap();
        let io = TestWorkspace::scratch_io();
        TestWorkspace { specs, dir, io }
    }

    pub fn scratch_io() -> IO<Vec<u8>, Vec<u8>> {
        let stdout = Vec::new();
        let stderr = Vec::new();
        tmpl::io::IO::new(stdout, stderr)
    }
}

impl Default for TestWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
