use colored::*;

fn main() {
    if let Err(err) = forge::run() {
        eprintln!("{}: {}", "Error".red().bold(), err)
    }
}
