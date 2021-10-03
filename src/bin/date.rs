use chrono::prelude::*;

fn main() {
    let dt = Local::now();
    println!("{}",dt.format("%Y/%m/%d %H:%M:%S %z"));
}