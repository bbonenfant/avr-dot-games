#![no_std]
#![no_main]
extern crate panic_halt;
use dot_games::games::SelectionScreen;


#[arduino_uno::entry]
fn main() -> ! {
    let mut components = dot_games::get_components();

    // Run the Selection Screen.
    let selected_game_loop = SelectionScreen::new().run(&mut components);

    // Run the Game Loop.
    selected_game_loop(components)
}