use std::process::{Command, Output, Child};

pub fn commander() -> Result<(), std::io::Error> {
    Command::new("echo").arg("Hello world!").spawn()?.wait();
    Ok(())
}