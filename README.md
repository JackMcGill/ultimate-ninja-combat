# Ultimate Ninja Combat

This is a re-write of one of my old Uni assignments, where we were tasked with creating a text-based game. My original
version can be found [here](/old-version.py) - this was one of the first programs I ever wrote.

**Why did you re-write an old Uni assignment?** The original assignment was to be written in Python. I'm interested in
learning Rust, and thought it would be a good first program to write, as it covers most of the basics of programming -
loops, variables, control-flow, user input, printing to the terminal.

Also, my original program had quite a few errors and will crash if you give it input it wasn't expecting at certain
points.
This was a fun excercise to apply what I've learned since I wrote the original one, making sure to account for
unexpected input, and writing all-round safer code with proper error-handling - Rust makes this stupidly trivial.

## To run

### Install Rust

Install Rust via Rustup on your machine if you haven't got it already:

#### Mac / Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows

Installation instructions on the Rust [website](https://www.rust-lang.org/learn/get-started).

Installing through Rustup will also give you Cargo, the Rust build tool + package manager

### Running

Clone the repository, and simply run `cargo run` in the terminal to get started.

## Assignment Description

_Paraphrased to make sure I'm not breaking any copyright rules_

You are required to create a text-based a game, where you play against the computer. The player will start the game with
a balance of $100. The player will choose a move from a list of six possibilities, and the computer will randomly choose
a move as well. The winner of that round will be announced followed by the updated balance (depending on whether the
player won or lost and how much they bet). The player should not be able to bet more than they have in their balance.

The list of moves (Computer moves are the top row, Player moves are the left column):

|                      | Punch of Fury | Kick of Punishment | Sword of Justice | Shuriken of Vengence | Nunchucks of Anger | Knife of Freedom |
|----------------------|---------------|--------------------|------------------|----------------------|--------------------|------------------|
| Punch of Fury        | Tie           | Player Loses       | Player Loses     | Player Wins          | Player Wins        | Player Loses     |
| Kick of Punishment   | Player Wins   | Tie                | Player Loses     | Player Loses         | Player Wins        | Player Loses     |
| Sword of Justice     | Player Wins   | Player Wins        | Tie              | Player Loses         | Player Loses       | Player Wins      |
| Shuriken of Vengence | Player Loses  | Player Wins        | Player Wins      | Tie                  | Player Loses       | Player Wins      |
| Nunchucks of Anger   | Player Loses  | Player Loses       | Player Wins      | Player Wins          | Tie                | Player Loses     |
| Knife of Freedom     | Player Wins   | Player Wins        | Player Loses     | Player Loses         | Player Wins        | Tie              |
|                      |               |                    |                  |                      |                    |

At the end of each round, the winner is declared and the updated balances are displayed.

When the game is over, or the player quits, the players name and balance should be displayed, as well as a history of
each round.