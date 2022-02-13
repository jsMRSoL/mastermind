// use anyhow::{anyhow, Result};
use rand::Rng;
use std::dbg;
use std::error::Error;
use std::io;
use std::str::FromStr;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
const MAX_TURNS: usize = 6;

fn main() {
    if let Err(e) = run() {
        println!("Application error: {:?}\n{}", e, e.to_string());
    };
}

fn run() -> Result<(), Box<dyn Error>> {
    start_round();
    let secret: Vec<Bobble> = set_up_pins();
    // dbg!(&secret);
    let mut guess = String::new();
    // println!("Your guess: {}", guess);
    for i in 0..MAX_TURNS {
        read_guess(&mut guess);
        // println!("Guess string: {}", guess);

        let mut guess_vec = guess
            .split(" ")
            .map(|c| c.trim().parse::<Bobble>())
            .collect::<Result<Vec<Bobble>, _>>()?;
        if guess_vec.len() > 6 {
            // println!("That's too many guesses! I'll just take your first six.");
            guess_vec.truncate(6);
        }
        // println!("Your guess: {:?}", &guess_vec);

        let mut right_col_pos: usize = 0;
        let mut right_col: usize = 0;
        let mut rest: usize = 6;
        for b in 0..6 {
            if &secret[b] == &guess_vec[b] {
                right_col_pos += 1;
            }
            if secret.contains(&guess_vec[b]) {
                right_col += 1;
            }
        }

        if right_col_pos == 6 {
            println!("Your guess: {:?}", &guess_vec);
            show_win();
            break;
        }
        rest = rest - right_col;
        right_col = right_col - right_col_pos;

        println!(
            "Guess {}: {:?}  Marks: {}{}{}",
            i + 1,
            &guess_vec,
            "Y".repeat(right_col_pos),
            "y".repeat(right_col),
            "-".repeat(rest)
        );

        if i == MAX_TURNS - 1 {
            println!("Better luck next time!");
            println!("The code: {:?}", &secret);
        }
        guess.clear();
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Bobble {
    Red,
    Green,
    Blue,
    Pink,
    Black,
    Yellow,
    White,
    Orange,
}

use mastermind::ParsePinError;
impl FromStr for Bobble {
    type Err = ParsePinError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Bobble::*;
        match s {
            "R" | "r" => Ok(Red),
            "G" | "g" => Ok(Green),
            "B" => Ok(Blue),
            "P" | "p" => Ok(Pink),
            "b" => Ok(Black),
            "Y" | "y" => Ok(Yellow),
            "W" | "w" => Ok(White),
            "O" | "o" => Ok(Orange),
            _ => Err(ParsePinError(s.to_owned())),
        }
    }
}

fn set_up_pins() -> Vec<Bobble> {
    use Bobble::*;
    let mut options = vec![Red, Green, Blue, Pink, Black, Yellow, White, Orange];
    let mut chosen: Vec<Bobble> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..6 {
        let idx = rng.gen_range(0..options.len());
        let choice = options.swap_remove(idx);
        chosen.push(choice);
    }
    chosen
}

fn start_round() {
    print!("{}", CLEAR);
    println!("{}", "Welcome to Mastermind Trainer!");
    println!("{}", "Red:  R/r, Green: G/g, Blue: B, Pink: P/p,");
    println!("{}", "Black:  b, Yellow: Y/y, White: W/w, Orange: O/o,");
    println!("{}", "Make your first guess!");
}

fn show_win() {
    println!("{}", "YOU WON!!!");
    println!("{}", "YOU are a MASTERMIND!");
}

fn read_guess(s: &mut String) {
    io::stdin().read_line(s).expect("Couldn't read guess.");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_pin_from_str() -> Result<(), anyhow::Error> {
//         let s = "P";
//         let pin: Pin = s.parse()?;
//         println!("{:?}", pin);
//         Ok(())
//     }
// }
