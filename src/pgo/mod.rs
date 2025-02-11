use colored::Colorize;

pub(crate) mod env;
pub mod instrument;
pub mod optimize;

pub fn llvm_profdata_install_hint() -> String {
    format!(
        "Try installing `llvm-profdata` using `{}` or build LLVM manually and \
add its `bin` directory to PATH.",
        "rustup component add llvm-tools-preview".blue()
    )
}

#[derive(Debug, Copy, Clone)]
pub enum CargoCommand {
    Build,
    Test,
    Run,
}

impl CargoCommand {
    pub fn to_str(&self) -> &str {
        match self {
            CargoCommand::Build => "build",
            CargoCommand::Test => "test",
            CargoCommand::Run => "run",
        }
    }
}
