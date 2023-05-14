use std::cmp::max;
use std::fmt::Display;
use std::{io, thread};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::connect4board::{Board, BOARD_COLS, EMPTY, Team, TEAM_O, team_to_char, TEAM_X};
use rand::prelude::*;

mod connect4board;
mod util;

const TWO_PLAYER: bool = false;       // whether or not the game is two player
const NUM_TEST_GAMES: usize = 50_000; // bigger => longer, better. smaller => shorter, less accurate

fn next_turn(team: Team) -> Team {
    match team  {
        TEAM_O => TEAM_X,
        TEAM_X => TEAM_O,
        _ => panic!("not a team! {}", team)
    }
}

// in-place DFS with a number of end configurations that are limited
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

//
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

//
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

fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
fn main() {

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
