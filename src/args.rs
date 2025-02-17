#[derive(Debug)]
pub enum Mode {
    Debug,
    Test,
    Release,
    ReleaseTest,
}

#[derive(Debug)]
pub struct Args {
    mode: Mode,
    cargo_flags: Vec<String>,
    rust_flags: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        Self {
            mode: Mode::Debug,
            cargo_flags: Vec::new(),
            rust_flags: Vec::new(),
        }
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn add_cargo_flag<F: AsRef<str>>(mut self, flag: F) -> Self {
        self.cargo_flags.push(flag.as_ref().to_string());
        self
    }

    pub(crate) fn cargoflags(&self) -> impl IntoIterator<Item = &str> {
        self.cargo_flags.iter().map(String::as_str)
    }
    
    pub(crate) fn rustflags(&self) -> impl IntoIterator<Item = &str> {
        let mode_flags: &[&str] = match self.mode {
            Mode::Debug => &[],
            Mode::Test => &["--test"],
            Mode::Release => &["--release"],
            Mode::ReleaseTest => &["--test", "--release"],
        };
        mode_flags.into_iter().map(|&s| s).chain(self.rust_flags.iter().map(String::as_str))
    }
}
