use crate::{
    Components,
    peripherals::DotScreen,
};

/// This trait exposes an interface to run games on the 8x8 LED Dot Screen.
pub trait Game {

    /// This method is called when the game is over.
    /// 
    /// When the game over state is complete, this method should return.
    /// 
    /// # Args
    /// * components - The peripheral components for the game display.
    fn game_over(&self, components: &mut Components);

    /// This method is called to begin the game-play.
    /// 
    /// This is expected to construct its own game loop. Once the game-play
    ///   ends, this method should return.
    /// 
    /// # Args
    /// * components - The peripheral components for the game display.
    fn play(&mut self, components: &mut Components);

    /// This method is called to reset the game to its initial state.
    /// 
    /// After this method is called, the game should be ready to be played again.
    fn reset(&mut self);

    /// This method returns the title screen for the game.
    /// 
    /// This method is non-static so that this trait can become a trait object.
    /// 
    /// # Returns
    /// The DotScreen object which displays as the title screen.
    fn title_screen(&self) -> &'static DotScreen;
}