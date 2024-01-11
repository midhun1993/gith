use std::env;
use std::process:: { Command};
use std::io::{Stdout};

fn get_command_pallet()-> Command{
    if cfg!(target_os = "windows") {
        return Command::new("cmd");
    } else {
        return Command::new("sh");
    }
}
fn is_git_installed() {
    let mut pallet = get_command_pallet();
    let output  = pallet.args(["git", "status"]).output().expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    is_git_installed();
   dbg!(args);
}
