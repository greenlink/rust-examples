use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{}", print_guessed_word(guess));

}

fn print_guessed_word(guessed_word: String) -> String {
    let mut guessed_phrase = String::from("You guessed: ");
    guessed_phrase.push_str(&guessed_word);
    guessed_phrase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_input_appears_on_final_result() {
        let guessed_word = String::from("carnival");
        let expect = String::from("You guessed: carnival");

        let result = print_guessed_word(guessed_word);
        assert_eq!(expect, result);
    }
}