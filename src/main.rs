use std::io;
use std::io::Write;

use rand::Rng;

static MOVES_LIST: [&str; 6] = ["Punch of Fury", "Kick of Punishment", "Sword of Justice", "Shuriken of Vengence", "Nunchucks of Anger", "Knife of Freedom"];

fn main() {
    let mut player_name: String = String::new();
    print!("Please enter your name: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut player_name).expect("Error reading input");
    let player_name = player_name.trim_end();
    println!("Welcome {}, this is Ultimate Ninja Combat!", player_name);
    instructions();

    loop {
        // get input
        let mut input: String = String::new();
        print!("[I]nstructions, [P]lay, [Q]uit: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error reading input");

        // let output: str;

        match &input.to_uppercase().trim_end() as &str {
            "I" => instructions(),
            "P" => break,
            "Q" => {
                println!("Quit");
                return;
            }
            _ => println!("The option you entered is invalid, please use one of the following:")
        }
    }
    game(&player_name);
}

fn get_move() -> usize {
    loop {
        let mut move_input: String = String::new();
        print!("Choose a move: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut move_input).expect("Error reading input");
        // Validate
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
                println!("Wrongk");
                continue;
            }
        }
    }
}

fn get_computer_move() -> usize {
    return rand::thread_rng().gen_range(0..6);
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

    return MATRIX[player][computer] as u8;
}

fn game(player_name: &str) {
    let mut computer_score: u8 = 0;
    let mut player_score: u8 = 0;

    loop {
        moves();
        // Moves selection
        let player_move: usize = get_move();
        let computer_move: usize = get_computer_move();

        println!("{} chose: {}", player_name, MOVES_LIST[player_move]);
        println!("Computer chose: {}", MOVES_LIST[rand::thread_rng().gen_range(0..6)]);

        let outcome = compare_moves(player_move, computer_move);

        match outcome {
            0 => println!("Win!"),
            1 => println!("Lose!"),
            2 => println!("Tie!"),
            _ => println!("You shouldn't ever be able to see this...")
        };

        // Menu selection
        loop {
            let mut input: String = String::new();
            print!("[I]nstructions, [P]lay, [Q]uit: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut input).expect("Error reading input");

            match &input.to_uppercase().trim_end() as &str {
                "I" => instructions(),
                "P" => break,
                "Q" => return,
                _ => println!("The option you entered is invalid, please use one of the following:")
            }
        }
    }
}

fn instructions() {
    println!("Instructions: You'll be fighting against the computer, and the winner gets bragging rights.");
    println!("To begin with, both you and the computer have $100 each.");
    println!("At the start of each round, you can place a bet in multiples of $5.");
    println!("After the bet, you'll be asked to choose from one of the moves below:");
    moves();
}

fn moves() {
    for (i, attack) in MOVES_LIST.iter().enumerate() {
        println!("[{}]: {}", i + 1, attack);
    }
}
