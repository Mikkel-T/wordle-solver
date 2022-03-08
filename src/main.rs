mod help;
mod input;
mod wordle;

fn main() {
    help::print();
    let data = String::from_utf8_lossy(include_bytes!("../words.txt"));
    let mut words: Vec<String> = data.lines().map(|i| i.to_string()).collect();
    loop {
        println!("");

        if words.is_empty() {
            println!("No more words to choose from, quitting program.");
            break;
        }

        let recommended = wordle::recommend(&words);
        println!(
            "Suggested word: \"{}\", word was chosen from {} words",
            recommended.to_uppercase(),
            words.len()
        );
        let (used_recommended, result, used_word) = input::get_result();
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
                help::print();
            }
            "a" | "all" => {
                for word in &words {
                    println!("{}", word.to_uppercase());
                }
            }
            _ => {
                if used_recommended {
                    words = wordle::filter(&words, recommended, result);
                } else {
                    words = wordle::filter(&words, used_word, result);
                }
            }
        }
    }
}
