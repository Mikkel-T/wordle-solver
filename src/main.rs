use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};

fn main() {
    print_help();
    let data = String::from_utf8_lossy(include_bytes!("../words.txt"));
    let mut words: Vec<String> = data.lines().map(|i| i.to_string()).collect();
    loop {
        let recommended = recommend(&words);
        println!(
            "Suggested word: \"{}\", word was chosen from {} words",
            recommended.to_uppercase(),
            words.len()
        );
        let result = get_result();
        match result.as_str() {
            "ccccc" => {
                println!("Solved!");
                break;
            }
            "q" | "quit" => {
                println!("Quitting");
                break;
            }
            "r" | "restart" => {
                println!("Restarting...",);
                words = data.lines().map(|i| i.to_string()).collect();
            }
            "i" | "invalid" => {
                println!(
                    "Removing the invalid word \"{}\"",
                    recommended.to_uppercase()
                );
                words.retain(|i| i != &recommended);
            }
            "h" | "help" => {
                print_help();
            }
            _ => {
                words = filter(&words, recommended, result);
            }
        }
    }
}

fn print_help() {
    println!("To enter a result, write 5 letters that repesent the colors from the result. The letters are: n - not in the word (Gray), w - wrong place (Yellow), c - correct (Green). For example, for 🟩⬜🟨🟨🟨 you can write \"cnwww\"");
    println!("Available commands:");
    println!("q | quit - Stops the program");
    println!("r | restart - Restarts the program");
    println!("i | invalid - Marks the current word as invalid and chooses a new one");
    println!("h | help - Shows the help dialog");
}

fn get_result() -> String {
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

fn filter(words: &Vec<String>, word: String, result: String) -> Vec<String> {
    let word = word.chars().collect::<Vec<char>>();
    let result = result.chars().collect::<Vec<char>>();
    let mut words: Vec<String> = words.into_iter().map(|i| i.to_string()).collect();
    for (i, c) in result.iter().enumerate() {
        match c {
            'n' => {
                words = words
                    .into_iter()
                    .filter(|w| !w.contains(&word[i].to_string()))
                    .collect();
            }
            'w' => {
                words = words
                    .into_iter()
                    .filter(|w| w.contains(&word[i].to_string()))
                    .filter(|w| w.chars().nth(i).unwrap() != word[i])
                    .collect();
            }
            'c' => {
                words = words
                    .into_iter()
                    .filter(|w| w.contains(&word[i].to_string()))
                    .filter(|w| w.chars().nth(i).unwrap() == word[i])
                    .collect();
            }
            _ => panic!("An error occurred while parsing an invalid result!"),
        }
    }
    return words;
}

fn recommend(words: &Vec<String>) -> String {
    let mut letters: HashMap<String, u128> = HashMap::new();
    for line in words {
        let mut chars = line.chars().collect::<Vec<char>>();
        let mut uniques = HashSet::new();
        chars.retain(|e| uniques.insert(e.clone()));

        for c in chars {
            let counter = letters.entry(String::from(c)).or_insert(0);
            *counter += 1;
        }
    }

    let mut ratings: HashMap<String, u128> = HashMap::new();

    for line in words {
        let mut percentages: Vec<u128> = Vec::new();
        let mut chars = line.chars().collect::<Vec<char>>();
        let mut uniques = HashSet::new();
        chars.retain(|e| uniques.insert(e.clone()));

        for c in &chars {
            percentages.push(*letters.get(&c.to_string()).unwrap());
        }

        ratings.insert(line.to_string(), percentages.iter().sum::<u128>());
    }

    let recommendation = ratings.iter().max_by_key(|a| a.1).unwrap().0;
    return recommendation.to_string();
}
