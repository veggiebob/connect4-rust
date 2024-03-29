use std::cmp::max;
use std::fmt::Display;
use std::{io, thread};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::connect4board::{Board, BOARD_COLS, BOARD_ROWS, EMPTY, Team, TEAM_O, team_to_char, TEAM_X};
use rand::prelude::*;

mod connect4board;
mod util;

#[cfg(test)]
mod test;

const TWO_PLAYER: bool = false;       // whether or not the game is two player
const NUM_TEST_GAMES: usize = 50_000; // bigger => longer, better. smaller => shorter, less accurate

/// Get the next turn
fn next_turn(team: Team) -> Team {
    match team  {
        TEAM_O => TEAM_X,
        TEAM_X => TEAM_O,
        _ => panic!("not a team! {}", team)
    }
}

/// in-place DFS with a number of end configurations that are limited
pub fn limited_dfs(board: &mut Board, turn: Team, num_left: &mut usize, rng: &mut ThreadRng) {
    if *num_left <= 0 {
        return;
    }
    if let Some(winner) = board.won() {
        *num_left -= 1;
        // won by person on previous turn
        println!("\n{}Game won by {}: {:?}", board, team_to_char(next_turn(turn)), winner);
        return;
    }

    if board.is_full() {
        println!("The final board is full! (not counting this one)\n{}", board);
    }

    let mut choices = (0..BOARD_COLS).into_iter().collect::<Vec<_>>();
    choices.shuffle(rng);
    for choice in choices {
        match board.drop(choice, turn) {
            Err(e) => continue,
            Ok(_) => {
                limited_dfs(board, next_turn(turn), num_left, rng);
                board.undrop(choice).unwrap(); // should work, because we just dropped it down there
            }
        }
        if *num_left <= 0 {
            return;
        }
    }
}

type GameResult = Team;

/// play a random game
pub fn play_random_game(board: &Board, turn: Team, rng: &mut ThreadRng) -> GameResult {
    let mut board = board.clone();
    let mut turn = turn;
    loop {
        if let Some((_p1, _p2, p)) = board.won() {
            return p;
        }
        if board.is_full() {
            return EMPTY;
        }
        let mut choices = board.valid_choices();
        let choice = choices.choose(rng).unwrap();
        board.drop(*choice, turn).unwrap();
        turn = next_turn(turn);
    }
}

// Run monte carlo simulation to determine best move
pub fn recommend_best_move(board: &mut Board, turn: Team, num_test_games: usize) -> usize {
    let moves = board.valid_choices();
    let mut outcomes = vec![];
    let test_games = num_test_games / moves.len();
    let (sender, receiver) = std::sync::mpsc::channel();
    for choice in moves.iter() {
        // play move
        board.drop(*choice, turn).unwrap();
        let nboard = board.clone();
        board.undrop(*choice).unwrap();
        let tx = sender.clone();
        let choice = *choice;
        thread::spawn(move || {
            let mut rng = thread_rng();
            // evaluate future games
            let mut win = 0;
            let mut lose_tie = 0;
            let opp = next_turn(turn);
            for _g in 0..test_games {
                if play_random_game(&nboard, opp, &mut rng) == turn {
                    win += 1;
                } else {
                    lose_tie += 1;
                }
            }

            // record results
            tx.send((choice, (win, lose_tie)));
        });
    }
    // so that the receiver actually stops
    std::mem::drop(sender);
    for result in receiver {
        outcomes.push(result);
    }
    // println!("outcomes of each game:\n{}",
    //          outcomes.iter()
    //              .map(|(choice, (win, lose))|
    //                  format!("{} => {}/{}", choice, win, win + lose))
    //              .collect::<Vec<_>>()
    //              .join("\n")
    // );
    match outcomes.iter().max_by_key(|(_c, (win, lose_tie))| ((*win as f32 / *lose_tie as f32) * 1000.) as usize) {
        Some((c, _)) => {
            *c
        },
        None => panic!("No moves left!")
    }
}

/// Get input from the user
fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() -> Result<(), ()> {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();
    return if let Some(arg) = args.get(1) {
        if arg == "single-turn" {
            single_turn()
        } else {
            eprintln!("Unknown argument {}", arg);
            Err(())
        }
    } else {
        interactive();
        Ok(())
    }
}

/// this is for api calls, so we always assume the user is X
/// and the AI player is O
fn single_turn() -> Result<(), ()> {
    // read BOARD_ROWS lines of BOARD_COLS characters
    let mut string = String::new();
    for _ in 0..BOARD_ROWS {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        string += &line;
    }
    let string = string.trim().to_string();
    let mut board = Board::parse(&string);

    // check and see if anyone has already won
    if let Some((p1, p2, team)) = board.won() {
        println!("x,y,team");
        println!("{},{},{}", p1.0, p1.1, team_to_char(team));
        println!("{},{},{}", p2.0, p2.1, team_to_char(team));
        return Ok(());
    }
    let choice = recommend_best_move(&mut board, TEAM_O, NUM_TEST_GAMES);
    board.drop(choice, TEAM_O).unwrap();

    // check and see if the AI just won
    if let Some((p1, p2, team)) = board.won() {
        println!("x,y,team");
        println!("{},{},{}", p1.0, p1.1, team_to_char(team));
        println!("{},{},{}", p2.0, p2.1, team_to_char(team));
        return Ok(());
    }
    println!("---\n{}", board.serialize());
    Ok(())
}

fn interactive() {
    let mut board = Board::init_empty();
    let mut turn = TEAM_X;
    if !TWO_PLAYER {
        println!("You are team X. Go first!");
    }
    loop {
        if board.is_full() {
            println!("Game over!");
            return;
        }
        if let Some((p1, p2, p)) = board.won() {
            println!("Game has been won by {}: {:?}", team_to_char(p), (p1, p2));
            return;
        }
        println!("{}It is {}'s turn.", board, team_to_char(turn));
        if TWO_PLAYER || turn == TEAM_X {
            loop {
                let user_input = input();
                match user_input.trim().parse::<usize>() {
                    Ok(num) => {
                        println!("You chose {}", num);
                        if num >= BOARD_COLS || num < 0 {
                            println!("Danny, you're gayyyy?!!??! My son is a fudge packer! What about the church?! You're gonna rot in hell for this >:( \
                                To not rot in hell, pick a number 0 - 6! ");
                            continue;
                        }
                        match board.drop(num, turn) {
                            Ok(_) => break,
                            Err(e) => println!("Can't drop there. Pick somewhere else.")
                        }
                    },
                    Err(e) => println!("Bad parse! {}", e)
                }
            }
        } else {
            let choice = recommend_best_move(&mut board, turn, NUM_TEST_GAMES);
            println!("AI plays {}", choice);
            board.drop(choice, turn).unwrap();
        }
        turn = next_turn(turn);
    }
}
