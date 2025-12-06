/// Return a tmpl::io::IO with Vec<u8>s instead of system stdout and stderr for testing output.
pub fn test_io() -> tmpl::io::IO<Vec<u8>, Vec<u8>> {
    let stdout = Vec::new();
    let stderr = Vec::new();
    tmpl::io::IO::new(stdout, stderr)
}

/// Creates a temporary directory and Specs struct for testing purposes
pub fn create_test_workspace() -> (tempfile::TempDir, tmpl::specs::Specs) {
    let temp = tempfile::tempdir().unwrap();
    let specs = tmpl::specs::Specs::new(temp.path()).unwrap();
    (temp, specs)
}
