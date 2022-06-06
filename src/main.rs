use std::env;
use std::io::{stderr, Write};
use std::process::{Command, Stdio, exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();
    let cmd_head: Vec<String> = args.drain(0..2).collect();

    let output = Command::new(&cmd_head[1])
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    stderr().write_all(&output.stderr)?;
    exit(output.status.code().ok_or("Failed to retrieve child exit code")?); 
}
