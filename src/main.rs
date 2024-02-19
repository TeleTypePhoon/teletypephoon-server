use std::{env, process::Command};

mod console;
mod server_core;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    if args.contains(&"--core-only".to_string()) {
        server_core::run(&args[1..].to_vec());
    } else {
        args.push(String::from("--core-only"));
        if let Ok(mut child) = Command::new(program).args(args[1..].to_vec()).spawn() {
            child.wait().expect("Server shut down");
        } else {
            panic!("Could not start core process");
        }
        console::run(&args);
    }
}
