use std::fs::File;
use std::env;
use std::path::Path;

fn usage(path: &String) {
    let cmdPath = Path::new(path);
    let command = cmdPath.file_stem().unwrap();
    println!("Usage: {}", command.to_string_lossy());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    usage(&args[0]);
}

