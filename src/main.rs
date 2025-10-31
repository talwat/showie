use showie::{render, Trim};
use std::{env, io, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("showie: error: no file path defined");
        exit(1);
    }

    let path = &args[1];
    let img = match image::ImageReader::open(path) {
        Ok(file) => file.decode().unwrap(),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("showie: error: file not found");
                exit(1);
            }
            _ => {
                eprintln!("showie: error: {:?}", err);
                exit(1);
            }
        },
    };

    let trimmed = img.trim();
    println!("{}", render(&trimmed));
}
