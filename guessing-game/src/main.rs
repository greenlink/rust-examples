use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        let result = get_guess_result(secret_number, guess);

        println!("{result}");
        if result == String::from("You win!") {
            break;
        }
    }
}

fn get_guess_result(secret_number: u32, guess: u32) -> String {
    let result:String;

    match guess.cmp(&secret_number) {
        Ordering::Less => result = String::from("Too small!"),
        Ordering::Greater => result = String::from("Too big!"),
        Ordering::Equal => result = String::from("You win!"),
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_guess_result_is_you_win() {
        let expected = String::from("You win!");
        let secret_number = 1;
        let guess = 1;

        let result = get_guess_result(secret_number, guess);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_guess_result_is_too_big() {
        let expected = String::from("Too big!");
        let secret_number = 1;
        let guess = 2;

        let result = get_guess_result(secret_number, guess);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_guess_result_is_too_small() {
        let expected = String::from("Too small!");
        let secret_number = 2;
        let guess = 1;

        let result = get_guess_result(secret_number, guess);

        assert_eq!(expected, result);
    }
}