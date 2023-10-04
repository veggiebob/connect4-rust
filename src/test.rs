use crate::connect4board::{Board, TEAM_O, TEAM_X};

// test adding tokens to a board, serializing, and parsing, and checking if equal
#[test]
fn test_adding_tokens() {
    let mut board = Board::init_empty();
    board.drop(0, TEAM_O).unwrap();
    board.drop(2, TEAM_O).unwrap();
    board.drop(4, TEAM_O).unwrap();
    board.drop(1, TEAM_X).unwrap();
    board.drop(2, TEAM_X).unwrap();
    board.drop(3, TEAM_X).unwrap();
    board.drop(5, TEAM_X).unwrap();

    let serialized = board.serialize();
    println!("{}", serialized);
    let parsed = Board::parse(&serialized);
    assert_eq!(board, parsed);
}