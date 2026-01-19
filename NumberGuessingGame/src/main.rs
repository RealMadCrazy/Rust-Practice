use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    help();

    loop {
        print!("\n{}", "Command: ".bright_yellow().bold());
        io::stdout().flush().unwrap();

        let mut input = wait_for_input();
        input = input.trim().to_lowercase();

        if input == "play" {
            play();
        } else if input == "exit" {
            exit();
        } else {
            println!("{}", "Invalid Command! ╰(*°▽°*)╯".bright_red());
        }
    }
}

fn play() {
    let secret_number = generate_random_number();

    println!(
        "{}",
        "\n🎯 I have chosen a number between 1 and 100. Can you guess it?".bright_cyan()
    );

    loop {
        print!("{}", "\nYour guess: ".bright_yellow().bold());
        io::stdout().flush().unwrap();

        let guess_input = wait_for_input();
        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "🚫 Please enter a valid number!".bright_red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "⬇️ Too small! Try a bigger one!".bright_blue()),
            Ordering::Greater => println!("{}", "⬆️ Too big! Try a smaller one!".bright_magenta()),
            Ordering::Equal => {
                println!(
                    "{}",
                    "🎉 You guessed it! Amazing! (〃￣︶￣)人(￣︶￣〃)"
                        .bright_green()
                        .bold()
                );
                break;
            }
        }
    }
}

fn generate_random_number() -> u32 {
    rand::rng().random_range(1..=100)
}

fn wait_for_input() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}

fn help() {
    println!(
        "{}\n{}",
        "------------------".bright_white(),
        "Welcome to the Number Guessing Game!".bright_cyan().bold()
    );
    println!(
        "{}\n{}\n{}",
        "Type Play to start the game (✿◡‿◡)".bright_green(),
        "Type Exit to exit the game O_O".bright_red(),
        "------------------".bright_white()
    );
}

fn exit() {
    println!("{}", "\nGoodbye! Thanks for playing! 👋".bright_yellow());
    process::exit(0);
}
