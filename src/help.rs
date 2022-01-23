pub fn print() {
    println!("To enter a result, write 5 letters that repesent the colors from the result. The letters are: n - not in the word (Gray), w - wrong place (Yellow), c - correct (Green). For example, for 🟩⬜🟨🟨🟨 you can write \"cnwww\"");
    println!("Available commands:");
    println!("q | quit - Stops the program");
    println!("r | restart - Restarts the program");
    println!("i | invalid - Marks the current word as invalid and chooses a new one");
    println!("h | help - Shows the help dialog");
}
