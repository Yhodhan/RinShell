pub enum Fork {
    Parent,
    Child,
}

pub fn fork() -> Result<Fork, String> {
    let pid: libc::pid_t = unsafe { libc::fork() };
    match pid {
        -1 => Err("Error in Fork".to_string()),
        0 => Ok(Fork::Child),
        _ => Ok(Fork::Parent),
    }
}
