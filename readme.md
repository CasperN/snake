# Snake

My first game - snake. It is still incomplete.


## Files
* `main.rs` holds the main game loop and passes `sdl2` events to `Controller`
* `controller.rs` handles parsing events and passes commands to the game of snake or the menu as appropriate
* `game_state.rs` contains the game logic.
* `game_render` draws the snake board
* `menu.rs` contains the main menu and renderer.


## TODOs
* Main menu
    * New game
        * Set speed
    * Previous high scores
    * Set board size
* Count down before game begins
