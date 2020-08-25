#![no_std]
#![no_main]
extern crate panic_halt;
use dot_games::games::SelectionScreen;


#[arduino_uno::entry]
fn main() -> ! {
    let mut components = dot_games::get_components();

    // Run the selection screen.
    let mut selection = SelectionScreen::new().run(&mut components);
    let game = selection.mut_ref_game();

    // Enter the game loop.
    loop {
        game.play(&mut components);
        game.game_over(&mut components);
        game.reset();
    }
}