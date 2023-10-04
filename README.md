# Connect4 Bot
Traditional board size, "X" vs. "O"

## Running

Interactive mode: `cargo run`
Single-Turn mode (used for API): `cargo run single-turn`

### Interactive mode
Play by entering column numbers, starting at 0, increasing 
going right (max column number is 6)

### Single-Turn mode
Board must be fed in as simple 7x6 characters, being "X", "O", or "_" (empty).
See [example file](test-materials/sample-board.txt)
We assume that X is the player using the API. The bot will play O, and place it on the board.
The output can be in one of two categories:
- It starts with "---". This means that the game is not over. What follows after this line is the state of the new board.
- Any other condition means the output is a CSV file with columns x, y, and team. Team will be the same in both
  (which is the winner) and the x and y will be coordinates of the two endpoints of the connect 4.

## Parameters

(see `main.rs`)

`TWO_PLAYER` - whether or not it's two player.

`NUM_TEST_GAMES` - number of random test games to be played in the
Montecarlo simulation. Balance between "accuracy" and speed. 
10,000 is reasonable.