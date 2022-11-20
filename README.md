# Ultimate Ninja Combat

This is a re-write of one of my old Uni assignments, where we were tasked with creating a text-based game. My original
version can be found [here](/old-version.py) - this was one of the first programs I ever wrote.

**Why did you re-write an old Uni assignment?** The original assignment was to be written in Python. I'm interested in
learning Rust, and thought it would be a good first program to write, as it covers most of the basics of programming -
loops, variables, control-flow, user input, printing to the terminal.

Also, my original program had quite a few errors and will crash if you give it input it wasn't expecting at certain
points.
This was a fun excercise to apply what I've learned since I wrote the original one, making sure to account for
unexpected input, and writing all-round safer code - Rust makes this stupidly trivial.

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