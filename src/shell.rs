use crate::fork::{fork, Fork};
use std::io::{self, Write};

fn exec(pid: Fork, cmd: &str) {
    match pid {
        Fork::Child => {
            println!("hi i am the child process and execute cmd {}", cmd);
            unsafe {
                libc::exit(0);
            }
        }
        Fork::Parent => (),
    }
}

pub fn shell() -> Result<(), String> {
    loop {
        print!("\x1b[0;35m~> \x1b[0m");

        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();

        let cmd = cmd
            .strip_suffix("\r\n")
            .or(cmd.strip_suffix("\n"))
            .ok_or("could not strip")?;

        if cmd == "exit" || cmd == "quit" {
            break;
        }
        // exec command
        match fork() {
            Err(err) => return Err(err),
            Ok(pid) => exec(pid, cmd),
        }
    }
    Ok(())
}
