mod fork;
mod shell;

fn main() -> Result<(), String> {
    shell::shell()
}
