use crate::fork::{fork, Fork};
use std::ffi::CString;
use std::io::{self, Write};
use std::ptr;

fn exec(cmd: Vec<String>) {
    unsafe {
        let args_cstrings = cmd
            .into_iter()
            .map(|arg| CString::new(arg))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut args: Vec<_> = args_cstrings.into_iter().map(|arg| arg.as_ptr()).collect();
        args.push(ptr::null());

        libc::execvp(args[0], args.as_ptr());
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
        let cmd = cmd
            .split_whitespace()
            .into_iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>();

        println!("cmd is: {:?}", cmd);

        let pid = fork()?;
        // exec command
        match pid {
            Fork::Child => exec(cmd),
            Fork::Parent => (),
        }
    }
    Ok(())
}
