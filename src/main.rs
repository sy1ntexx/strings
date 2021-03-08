use std::{io::Read};

fn main() {
    let mut min_len = 7;
    let args = std::env::args().collect::<Vec<String>>();
    // Verifying that we supplied a required filename
    if args.len() < 2 {
        println!("Usage: {} <filename> [minimum_length]", args.get(0).unwrap());
        return;
    }
    // Checks if user supplied minimum length
    if args.len() == 3 {
        // Need to handle parsing error
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
    // Opening 2 argumet (1 one is always path to executable)
    let f  = std::fs::File::open(args.get(1).unwrap());
    match f {
        Ok(mut f) => {
            // If all good parsing
            parse_strings(&mut f, min_len);
        }
        // Checks if file is openned successfully
        Err(ex) => {
            println!("Opening file failed: {}", ex);
        }
    }
}

fn parse_strings(f: &mut std::fs::File, min_len: i32) {
    // File length in bytes
    let mut length;
    // Buffer for data read from the file
    let mut data: [u8; 16] = [0; 16];

    // Index of found string
    let mut index = 0;
    // Current seeked length from file start
    let mut current = 0;
    // Shows current state of parser
    let mut capture = false;

    // Buffer for string
    let mut string = String::new();

    loop {
        // Read 16 bytes to data buffer
        length = f.read(&mut data).unwrap();
        if length == 0 {
            break;
        }

        // Iterate over all bytes
        for b in data.iter() {
            current += 1;

            let b = *b;
            // ASCII is always lower than 128
            if b > 0x7F {
                capture = false;
                continue;
            }

            if b == 0 {
                // If byte is zero and current string length greater then min_length print it
                if capture && current - index >= min_len {
                    println!("{}", string);
                }
                // Stop capturing string and reset it if the byte is zero anyway
                capture = false;
                string.clear();
            } else {
                // If we are not capturing but the byte is ok then begin
                if !capture {
                    capture = true;
                    index = current;
                    string.clear();
                }
            }
            // If byte is in range of ASCII normal symbols then push char to the string buffer
            if capture && (b > 0x21 && b < 0x7E) {
                string.push(b as char);
            } else {
                // If byte is garbage let's reset
                capture = false;
                string.clear();
            }
        }
    }
}