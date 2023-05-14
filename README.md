# Connect4 Bot
Traditional board size, "X" vs. "O"

## Running
`cargo run`

Play by entering column numbers, starting at 0, increasing 
going right (max column number is 6)

## Parameters

(see `main.rs`)

`TWO_PLAYER` - whether or not it's two player.

`NUM_TEST_GAMES` - number of random test games to be played in the
Montecarlo simulation. Balance between "accuracy" and speed. 
10,000 is reasonable.