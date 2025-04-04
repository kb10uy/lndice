use crate::parser::parse_command;

#[unsafe(no_mangle)]
pub fn run_parser() {
    parse_command("").expect("test");
}
