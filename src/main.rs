fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let result = std::io::stdin().read_line(&mut guess);
    result.expect("Failed to read line");

    println!("You guessed: {}", guess);
}
