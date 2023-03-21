use std::io;

fn main() {
    println!("Enter a word or phrase:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome", input);
    } else {
        println!("'{}' is not a palindrome", input);
    }
}

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}
