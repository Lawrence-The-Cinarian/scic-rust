use std::io::{self, Write};

pub fn clear_terminal() {
    println!("\x1B[3J\x1B[H\x1B[2J");
    io::stdout().flush().unwrap();
}

pub fn read_string(message: &str) -> String {
    let mut string = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    let string = string.trim().to_lowercase();
    string
}

pub fn read_number(message: &str) -> (f64, u64) {
    let mut string = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    let fvalue: f64 = string.trim().parse().unwrap();
    let svalue: u64 = fvalue as u64;
    (fvalue, svalue)
}
