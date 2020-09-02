use crate::{
    common::Direction,
    peripherals::{DotScreen, JoyStickSignal, InputSignal}
};

const NUMBER_OF_GAMES: usize = 1;

type GameLoop = fn(crate::Components) -> !;


/// Structure used to select the game to be played.
pub struct SelectionScreen {
    /// This is an array of (&TitleScreen, GameLoop) tuples.
    games: [(&'static DotScreen, GameLoop); NUMBER_OF_GAMES],
    /// The current index of the selection (indexing over the games array).
    index: usize,
}


impl SelectionScreen {

    /// Creates a new SelectionScreen object.
    pub fn new() -> Self {
        let games: [(&'static DotScreen, GameLoop); NUMBER_OF_GAMES] = [
            (&super::snake::TITLE_SCREEN, super::snake::snake_game_loop),
        ];
        Self { games, index: 0 } 
    }

    /// Gets the title screen DotScreen object for the current game.
    fn current_title_screen(&mut self) -> &DotScreen {
        self.games[self.index].0
    }

    /// Move the selection screen to the next game.
    fn next(&mut self) {
        self.index = (self.index + 1) % NUMBER_OF_GAMES;
    }

    /// Move the selection screen to the previous game.
    fn prev(&mut self) {
        self.index = (self.index - 1) % NUMBER_OF_GAMES;
    }

    /// Select the previous game. 
    /// 
    /// This consumes the SelectionScreen object, returning the GameLoop that 
    ///   runs the selected game.
    fn select(self) -> GameLoop {
        self.games[self.index].1
    }

    /// Run the Selection Screen.
    /// 
    /// This consumes the SelectionScreen object, returning the selected GameLoop
    ///   that runs the selected game.
    /// This will endlessly loop, reacting to inputs from the JoyStick peripheral.
    pub fn run(mut self, components: &mut crate::Components) -> GameLoop {
        const NEW_SELECTION_DELAY: u16 = 250;
        components.display.show(self.current_title_screen());
        return loop {
            match components.analog.poll_joystick_until_any() {
                InputSignal::JoyStick(signal) => {

                    // If the JoyStick button is pressed, return the GameLoop that runs the selected game.
                    if let JoyStickSignal { button: true, .. } = signal { break self.select() }

                    // If a horizontal direction is registered, change the current selection.
                    match signal.to_single_direction() {
                        Some(Direction::Left) => { 
                            self.prev();
                            components.display.show(self.current_title_screen());
                            arduino_uno::delay_ms(NEW_SELECTION_DELAY);
                        }
                        Some(Direction::Right) => {
                            self.next();
                            components.display.show(self.current_title_screen());
                            arduino_uno::delay_ms(NEW_SELECTION_DELAY);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}