use std::env;
use std::fs::File;
use std::io::stdout;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::exit;

const BUFFER_SIZE: usize = 4096;

fn usage(path: &String) {
    let cmd_path = Path::new(path);
    let command = cmd_path.file_stem().unwrap();
    println!("Usage: {}", command.to_string_lossy());
    exit(1);
}

fn main() {
    let cmd = env::args().nth(0).unwrap();
    let files: Vec<String> = env::args().skip(1).collect();
    if files.is_empty() {
        usage(&cmd);
    }

    let mut writer = stdout();

    for file in files {
        let f = match File::open(&file) {
            Err(why) => panic!("Could't open {}: {}", file, why),
            Ok(c) => c,
        };
        let mut reader = BufReader::new(&f);
        let mut buf = [0; BUFFER_SIZE];

        loop {
            let n = reader.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            writer.write_all(&buf[..n]).unwrap();
        }
    }
}
