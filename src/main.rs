use std::{io::Read};

fn main() {
    let mut min_len = 7;
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <filename> [minimum_length]", args.get(0).unwrap());
        return;
    }
    if args.len() == 3 {
        match args.get(2).unwrap().parse::<i32>() {
            Ok(val) => {
                min_len = val;
            }
            Err(_) => {
                println!("Invalid minimum length supplied. Aborting...");
                return;
            }
        }
    }
    let f  = std::fs::File::open(args.get(1).unwrap());
    match f {
        Ok(mut f) => {
            parse_strings(&mut f, min_len);
        }
        Err(ex) => {
            println!("Opening file failed: {}", ex);
        }
    }
}

fn parse_strings(f: &mut std::fs::File, min_len: i32) {
    let mut length;
    let mut data: [u8; 16] = [0; 16];

    let mut index = 0;
    let mut current = 0;
    let mut capture = false;

    let mut string = String::new();

    loop {
        length = f.read(&mut data).unwrap();
        if length == 0 {
            break;
        }

        for b in data.iter() {
            current += 1;

            let b = *b;
            if b > 0x7F {
                capture = false;
                continue;
            }

            if b == 0 {
                if capture && current - index >= min_len {
                    println!("{}", string);
                }
                capture = false;
                string = String::new();
            } else {
                if !capture {
                    capture = true;
                    index = current;
                    string = String::new();
                }
            }
            if capture && (b > 0x21 && b < 0x7E) {
                string.push(b as char);
            } else {
                capture = false;
                string = String::new();
            }
        }
    }
}