use rand::Rng;

fn print_intruction() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have 5 chances to guess the correct number");
    println!("\nPlease select the difficulty level:");
    println!("1. Easy (10 chances)");
    println!("2. Medium (5 chances)");
    println!("3. Hard (3 chances)");
}

fn handle_choice(choice: u32) -> u32 {
    match choice {
        1 => {
            println!("Great! You have selected Easy difficulty");
            10
        }
        2 => {
            println!("Great! You have selected Medium difficulty");
            5
        }
        3 => {
            println!("Great! You have selected Hard difficulty");
            3
        }
        _ => {
            println!("Invalid choice. Setting difficulty to Medium");
            5
        }
    }
}

fn handle_guess(guess: u32, secret_number: u32, attempt: u32) -> bool {
    if guess == secret_number {
        println!(
            "Congratulations! You guessed the correct number in {} attempts",
            attempt
        );
        true
    } else if guess < secret_number {
        println!("Incorrect! The number is greater than {}", guess);
        false
    } else {
        println!("Incorrect! The number is less than {}", guess);
        false
    }
}

fn main() {
    print_intruction();

    let mut s = String::new();
    println!("Enter your choice: ");

    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    let choice: u32 = s.trim().parse().expect("Please enter a number");

    let chances = handle_choice(choice);

    let secret_number = rand::thread_rng().gen_range(1..100);

    for i in 1..=chances {
        println!("\nPlease enter your guess: ");
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if let Ok(guess) = guess.trim().parse::<u32>() {
            if handle_guess(guess, secret_number, i) {
                break;
            }
        } else {
            println!("Please enter a valid number");
        }

        if i == chances {
            println!("Sorry! You have exhausted all your chances");
            println!("The correct number was: {}", secret_number);
        }
    }
}
