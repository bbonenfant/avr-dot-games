use crate::{
    common::Direction,
    peripherals::{DotScreen, JoyStickSignal, InputSignal}
};
use super::{Game, SnakeGame};

const NUMBER_OF_GAMES: usize = 1;


/// Enumeration of the all of the DotGames.
/// 
/// This is used in order to avoid using trait objects that have
///   Sized issues.
pub enum DotGame {
    // Empty option. This is used internally. 
    _Empty,
    
    // The Snake game.
    Snake(SnakeGame)
}

impl DotGame {
    /// Get a mutable reference to the inner Game object of all
    ///   (reachable) branches of the DotGame enumeration.
    pub fn mut_ref_game<'a>(&'a mut self) -> &'a mut dyn Game {
        match self {
            DotGame::Snake(game) => { game },
            DotGame::_Empty => { panic!("Reached unreachable branch of game selection. ") }
        }
    }
}


/// Structure used to select the DotGame to be played.
pub struct SelectionScreen {
    // This is an array of DotGame enums as opposed to Game trait objects
    //   due to Sized issues.
    games: [DotGame; NUMBER_OF_GAMES],
    // The current index of the selection (indexing over the games array).
    index: usize,
}



impl SelectionScreen {

    /// Creates a new SelectionScreen object.
    pub fn new() -> Self {
        let games = [
            DotGame::Snake(SnakeGame::new())
        ];
        Self { games, index: 0 } 
    }

    /// Pass through function to get the title screen DotScreen object for the current game.
    fn current_title_screen(&mut self) -> &DotScreen {
        self.games[self.index].mut_ref_game().title_screen()
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
    /// This consumes the SelectionScreen object, returning the selected DotGame object.
    fn select(mut self) -> DotGame {
        core::mem::replace(&mut self.games[self.index], DotGame::_Empty)
    }

    /// Run the Selection Screen.
    /// 
    /// This consumes the SelectionScreen object, returning the selected DotGame object.
    /// This will endlessly loop, reacting to inputs from the JoyStick peripheral.
    pub fn run(mut self, components: &mut crate::Components) -> DotGame {
        components.display.show(self.current_title_screen());
        return loop {
            match components.analog.poll_joystick_until_any() {
                InputSignal::JoyStick(signal) => {

                    // If the JoyStick button is pressed, return the selected DotGame.
                    if let JoyStickSignal { button: true, .. } = signal { break self.select() }

                    // If a horizontal direction is registered, change the current selection.
                    match signal.to_single_direction() {
                        Some(Direction::Left) => { 
                            self.prev();
                            components.display.show(self.current_title_screen());
                        }
                        Some(Direction::Right) => {
                            self.next();
                            components.display.show(self.current_title_screen());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}