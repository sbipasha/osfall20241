use std::io;
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    } else if guess > secret {
        return 1;
    } else {
        return -1;
    }
}
fn main() {
    let mut number = 0;
    let mut secret_num = 7;
    loop {
        println!("Guess: ");
        let mut enter_guess = String::new();
        io::stdin()
            .read_line(&mut enter_guess)
            .expect("Failed to read.");
        let guesses: i32 = enter_guess.trim().parse().expect("Put a valid integer.");
        let result = check_guess(guesses, secret_num);
        number += 1;
        if result == 0 {
            println!("Correct.");
            break;
        } else if result == 1 {
            println!("Too high. ");
        } else if result == -1 {
            println!("Too low. ");
        }
    }
    println!("Guess count: {}", number);
}
