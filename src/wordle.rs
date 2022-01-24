use std::collections::{HashMap, HashSet};

pub fn filter(words: &Vec<String>, word: String, result: String) -> Vec<String> {
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

pub fn recommend(words: &Vec<String>) -> String {
    let mut letters: HashMap<String, u128> = HashMap::new();
    for line in words {
        for c in unique(line) {
            let counter = letters.entry(String::from(c)).or_insert(0);
            *counter += 1;
        }
    }

    let mut ratings: HashMap<String, u128> = HashMap::new();

    for line in words {
        let mut percentages: Vec<u128> = Vec::new();

        for c in unique(line) {
            percentages.push(*letters.get(&c.to_string()).unwrap());
        }

        ratings.insert(line.to_string(), percentages.iter().sum::<u128>());
    }

    let recommendation = ratings.iter().max_by_key(|a| a.1).unwrap().0;
    return recommendation.to_string();
}

pub fn unique(line: &String) -> Vec<char> {
    let mut chars = line.chars().collect::<Vec<char>>();
    let mut uniques = HashSet::new();
    chars.retain(|e| uniques.insert(e.clone()));
    return chars;
}
