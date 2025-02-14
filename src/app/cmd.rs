use notblib::{get, stat};

pub fn run() -> Result<(), crate::AnyError> {
    stat::warning("Welcome to CWE client!");
    loop {
        stat::process("Enter command. For help use 'help'");
        let input = get::input("- ")?;
    }
}
