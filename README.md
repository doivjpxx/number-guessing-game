## Guessing Game

### Description
This is a simple command-line guessing game written in Rust. The objective of the game is to guess a randomly generated number within a specified range and number of attempts.

### How to Play
1. Start the Game: Run the program, and you will be prompted to enter your choice.
2. Enter Your Choice: Input a number to determine the number of chances you will get to guess the secret number.
3. Guess the Number: The program will generate a random number between 1 and 100. You will then be prompted to guess the number within the number of chances you specified.
**Hints**: After each guess, the program will inform you whether the secret number is greater or less than your guess.
Win or Lose: If you guess the number correctly within the given chances, you win. If you exhaust all your chances without guessing correctly, you lose, and the correct number will be revealed.

### How to Run
1. Clone the repository.
2. Navigate to the project directory.
3. Run the following command:
```sh
cargo run
```

### Example
```sh
Enter your choice: 
5

Please enter your guess: 
50
Incorrect! The number is greater than 50

Please enter your guess: 
75
Incorrect! The number is less than 75

Please enter your guess: 
60
Incorrect! The number is greater than 60

Please enter your guess: 
70
Incorrect! The number is less than 70

Please enter your guess: 
65
Congratulations! You guessed the correct number in 5 attempts.
```