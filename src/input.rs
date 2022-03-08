use regex::Regex;

use std::io::{stdin, stdout, Write};

pub fn get_result() -> (bool, String, String) {
    print!("Did you use the recommended word? [y/n] ");
    let mut used_recommended = true;
    let mut used_recommended_temp = String::new();
    loop {
        stdout().flush().ok().expect("Could not flush stdout");
        used_recommended_temp.clear();
        stdin().read_line(&mut used_recommended_temp).expect("Failed to read line");
    
        used_recommended_temp = used_recommended_temp.trim().to_string();
    
        if used_recommended_temp.to_lowercase() == "y" || used_recommended_temp.to_lowercase() ==  "n" {
            if used_recommended_temp.to_lowercase() == "n" {
                used_recommended = false;
            }
            break;
        } else {
            print!("Invalid answer, try again: ");
        }
    }

    let mut used_word = String::new();
    if !used_recommended {
        let good_word = Regex::new(r"^\w{5}$").unwrap();
        print!("What word did you use? ");
        loop {
            stdout().flush().ok().expect("Could not flush stdout");
            used_word.clear();
            stdin().read_line(&mut used_word).expect("Failed to read line");
    
            used_word = used_word.trim().to_string();
    
            if good_word.is_match(&used_word) {
                break;
            } else {
                print!("Invalid word, try again: ");
            }
        }
    }   

    let good_input = Regex::new(r"^(r|restart|q|quit|i|invalid|h|help|a|all|[nwc]{5})$").unwrap();
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

    return (used_recommended, result, used_word);
}
