use std::{cmp, io};
use std::cmp::Ordering;
use std::io::Write;

use rand::Rng;

struct Round {
    round: u8,
    player_balance: u8,
    computer_balance: u8,
    outcome: u8,
}

static MOVES_LIST: [&str; 6] = ["Punch of Fury", "Kick of Punishment", "Sword of Justice",
    "Shuriken of Vengence", "Nunchucks of Anger", "Knife of Freedom"];

fn main() {
    let mut player_name: String = String::new();
    print!("Enter your name: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut player_name).expect("Error reading input");
    let player_name = player_name.trim_end();
    println!("Welcome {}, this is Ultimate Ninja Combat!", player_name);
    // instructions();

    loop {
        // get input
        let mut input: String = String::new();
        print!("[I]nstructions, [P]lay, [Q]uit: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error reading input");
        println!();
        // let output: str;

        match input.to_uppercase().trim_end() as &str {
            "I" => instructions(),
            "P" => break,
            "Q" => {
                println!("Quit");
                return;
            }
            _ => println!("The option you entered is invalid, please use one of the following:")
        }
    }
    game(player_name);
}

fn get_move() -> usize {
    loop {
        let mut move_input: String = String::new();
        print!("Choose your move: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut move_input).expect("Error reading input");
        println!();
        let player_move: u32 = match move_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, you dummy");
                continue;
            }
        };
        match player_move {
            1..=6 => return (player_move - 1) as usize,
            _ => {
                println!("That's not a valid move, dummy");
                continue;
            }
        }
    }
}

fn compare_moves(player: usize, computer: usize) -> u8 {
    const PUNCH_OUTCOMES: [i32; 6] = [2, 1, 1, 0, 0, 1];
    const KICK_OUTCOMES: [i32; 6] = [0, 2, 1, 1, 0, 1];
    const SWORD_OUTCOMES: [i32; 6] = [0, 0, 2, 1, 1, 0];
    const SHURIKEN_OUTCOMES: [i32; 6] = [1, 0, 0, 2, 1, 0];
    const NUNCHUCK_OUTCOMES: [i32; 6] = [1, 1, 0, 0, 2, 1];
    const KNIFE_OUTCOMES: [i32; 6] = [0, 0, 1, 1, 0, 2];
    const MATRIX: [[i32; 6]; 6] = [PUNCH_OUTCOMES, KICK_OUTCOMES, SWORD_OUTCOMES, SHURIKEN_OUTCOMES,
        NUNCHUCK_OUTCOMES, KNIFE_OUTCOMES];

    MATRIX[player][computer] as u8
}

fn bet(player: u8, computer: u8) -> u8 {
    loop {
        let mut input: String = String::new();
        let min: u8 = cmp::min(player, computer);
        print!("How much would you like to bet? (bet can only be in multiples of 5, and must be less than ${}): $", min);
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error reading input");
        let player_bet: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, you dummy");
                continue;
            }
        };
        match player_bet % 5 {
            0 => {
                match player_bet.cmp(&min) {
                    Ordering::Greater => {
                        println!("Your bet can not be greater than ${}, bet again", min)
                    }
                    _ => return player_bet,
                }
            }
            _ => {
                println!("Not a multiple of 5, try again you idiot");
                continue;
            }
        };
    }
}

fn balance(player: u8, computer: u8) {
    println!("Your new balance: ${} | Computer's new balance: ${}", player, computer);
    println!();
}

fn game(player_name: &str) {
    let mut history: Vec<Round> = Vec::new();

    let mut player_balance: u8 = 100;
    let mut computer_balance: u8 = 100;
    let mut round: u8 = 0;

    loop {
        moves();
        round += 1;
        println!("ROUND {}", round);
        println!("{}'s balance: ${} | Computers balance: ${}", player_name, player_balance, computer_balance);

        let bet: u8 = bet(player_balance, computer_balance);

        let player_move: usize = get_move();
        let computer_move: usize = rand::thread_rng().gen_range(0..6);

        println!("{} chose: {}", player_name, MOVES_LIST[player_move]);
        println!("Computer chose: {}", MOVES_LIST[computer_move]);

        let outcome = compare_moves(player_move, computer_move);

        match outcome {
            0 => {
                println!("You win this round!");
                player_balance += bet;
                computer_balance -= bet;
                println!();
                balance(player_balance, computer_balance);
            }
            1 => {
                println!("You lose this round!");
                player_balance -= bet;
                computer_balance += bet;
                println!();
                balance(player_balance, computer_balance);
            }
            _ => {
                println!("This round is a tie!");
            }
        };

        history.push(Round {
            round,
            player_balance,
            computer_balance,
            outcome,
        });

        if player_balance == 0 || computer_balance == 0 {
            print!("Game Over");
            if player_balance == 0 {
                println!(" - the computer won");
            } else {
                println!(" - {} won", player_name);
            }
            print_score(&history, player_name);
            return;
        }

        // Menu selection
        loop {
            let mut input: String = String::new();
            print!("[C]ontinue, [I]nstructions [Q]uit: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut input).expect("Error reading input");

            match input.to_uppercase().trim_end() as &str {
                "I" => instructions(),
                "C" => break,
                "Q" => {
                    println!("Thanks for playing {}! Your final balance was ${}", player_name, player_balance);
                    print_score(&history, player_name);
                    return;
                }
                _ => println!("The option you entered is invalid, please use one of the following:")
            }
        }
    }
}

fn print_score(history: &Vec<Round>, player_name: &str) {
    let mut input: String = String::new();
    print!("Would you like to see a breakdown of the rounds? [Y]es [N]o: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading input");

    match input.to_uppercase().trim_end() as &str {
        "Y" => {
            for round in history {
                println!();
                println!("After Round {} / {}:", round.round, history.len());
                println!("{}'s balance: ${} | Computer's balance: ${}", player_name, round.player_balance, round.computer_balance);
                match round.outcome {
                    0 => println!("You won round {}", round.round),
                    1 => println!("You lost round {}", round.round),
                    _ => println!("Round {} was a tie", round.round)
                }
                let divider_length: usize = player_name.len() + 42;
                for _ in 0..divider_length {
                    print!("*"); // 42
                }
                println!();
            }
        }
        _ => println!("Bye."),
    };
    println!();
}

fn instructions() {
    println!("Instructions: You'll be fighting against the computer, and the winner gets bragging rights.");
    println!("To begin with, both you and the computer have $100 each.");
    println!("At the start of each round, you can place a bet in multiples of $5.");
    println!("After the bet, you'll be asked to choose from one of the moves below:");
    moves();
}

fn moves() {
    println!("List of moves:");
    for (i, &attack) in MOVES_LIST.iter().enumerate() {
        println!("[{}]: {}", i + 1, attack);
    }
    println!();
}
