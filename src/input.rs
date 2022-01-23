use regex::Regex;

use std::io::{stdin, stdout, Write};

pub fn get_result() -> String {
    let good_input = Regex::new(r"^(r|restart|q|quit|i|invalid|h|help|[nwc]{5})$").unwrap();
    print!("Input result: ");
    let mut result = String::new();
    loop {
        stdout().flush().ok().expect("Could not flush stdout");
        result.clear();
        stdin().read_line(&mut result).expect("Failed to read line");

        result = result.trim().to_string();

        if good_input.is_match(&result) {
            break;
        } else {
            print!("Invalid result, try again: ");
        }
    }

    return result;
}