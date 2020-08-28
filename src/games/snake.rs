use crate::{Components, Direction};
use crate::peripherals::{Dot, DotScreen, InputSignal, JoyStickSignal};
use super::Game;

// Constants for the Snake game.
//   The x-coordinate of the egg starting location.
const EGG_START_X: usize = 1;
//   The y-coordinate of the egg starting location.
const EGG_START_Y: usize = DotScreen::WIDTH - 2;
//   The y-index (row) along which the snake starts.
const SNAKE_START_Y: usize = DotScreen::WIDTH / 2;
//   The stating length of the snake.
const START_LENGTH: usize = (DotScreen::WIDTH / 2) - 1;
//   The initial polling interval for the SnakeGame.
const INITIAL_POLL_INTERVAL: usize = 500;

/// A segment represents a segment of the Snake.
/// 
/// This is fully described by a Dot (the position on the screen)
///   and a Direction (indicating where the segment will be next).
#[derive(Copy, Clone)]
struct Segment {
    direction: Direction,
    position: Dot,
}

impl Segment {

    /// Create a new Segment object, from an x-coordinate, y-coordinate, and direction.
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        let position = Dot { x, y };
        Segment { direction, position }
    }

    /// Create a new Segment, which represents where the Snake will be
    ///   at the next game tick.
    /// 
    /// The direction of the segment remains constant.
    /// Additionally, the output Segment is constrained to be within the Dot grid.
    fn next(&self) -> Self {
        let position = match self.direction {
            Direction::Left=> { self.position.left() },
            Direction::Right => { self.position.right() },
            Direction::Up => { self.position.up() },
            Direction::Down => { self.position.down() },
        };
        Self { direction: self.direction, position }
    }
}


/// An enumeration of the results of the Snake::slither function.
enum SlitherResult {
    // The Snake moved successfully, but no egg was eaten.
    // The inner Segment is the last part of the Tail that was dropped.
    Moved(Segment),
    // The Snake moved successfully and the egg was eaten.
    EggEaten,
    // The Snake collided with itself or the wall.
    Collision,
}


// The Tail is implemented as an ArrayDeque object from the arraydeque crate.
type Tail = arraydeque::ArrayDeque<[Segment; DotScreen::TOTAL_DOTS], arraydeque::Wrapping>;

/// The Snake object. This is the character that the player controls.
/// 
/// This object is described by the Head, the part that the player controls,
///   and the Tail which follows in the tracks of the Head.
struct Snake {
    head: Segment,
    tail: Tail,
}

impl Snake {

    /// Constructs a new Snake object. 
    /// 
    /// This Snake is always in the same place, middle of the screen with tail extended
    ///   to the left and headed in the rightward direction, and the same length length,
    ///   one less than half of the screen width.
    fn new() -> Self {
        // Initialize an empty Snake.
        let head = Segment::new(0, 0, Direction::Up);
        let tail: Tail  = arraydeque::ArrayDeque::new();
        let mut snake = Snake { head, tail };

        // Initialize and return.
        snake.init();
        return snake
    }

    /// Initializes the Snake object.
    /// 
    /// This method can be used to "reset" the Snake to the original position.
    fn init(&mut self) {
        self.tail.clear();
        self.head = Segment::new(START_LENGTH, SNAKE_START_Y, Direction::Right);
        self.tail.push_back(Segment::new(START_LENGTH - 1, SNAKE_START_Y, Direction::Right));
        self.tail.push_back(Segment::new(START_LENGTH - 2, SNAKE_START_Y, Direction::Right));
    }

    /// Checks if the Snake has collided with itself or the wall.
    /// 
    /// If the Snake has collided with the wall, then its Head would not have moved,
    ///   meaning that the first segment of the snake is the same as the Head.
    /// 
    /// # Returns
    /// The determination of if a collision has occurred.
    fn check_collision(&self) -> bool { 
        for segment in self.tail.iter() {
            if self.head.position == segment.position {
                return true
            }
        }
        return false
    }

    /// Gets the length of the Snake.
    fn get_length(&self) -> usize {
        self.tail.len() + 1
    }

    /// Sets the direction of the Snake
    /// 
    /// This sets the direction of the Head of the Snake.
    /// The direction is not allowed to be backwards, since if the Snake turned
    ///   around, there would be a collision. This choice improves game play.
    fn set_direction(&mut self, dir: Direction) {
        if dir.opposite() != self.head.direction {
            self.head.direction = dir;
        }
    }

    /// Moves the snake to the next position.
    /// 
    /// Returns a SlitherResult indicating one of the following:
    ///   * The Snake ate the egg,
    ///   * The Snake collided with either itself or the wall,
    ///   * The Snake moved to the next space, but did not eat the egg.
    fn slither(&mut self, egg: &Dot) -> SlitherResult {
        self.tail.push_front(self.head); 
        self.head = self.head.next();
        return
            if self.head.position == *egg { SlitherResult::EggEaten }
            else {
                let dropped_segment = self.tail.pop_back().unwrap();
                if self.check_collision() { SlitherResult::Collision }
                else { SlitherResult::Moved(dropped_segment) }
            } 
    }
}

/// The SnakeGame object.
pub struct SnakeGame {
    // The Egg that the Snake is trying to eat.
    egg: Dot,
    // The character that the player controls.
    snake: Snake,
    // The screen depicting the current state of the game.
    screen: DotScreen,
    // The interval to poll for user input.
    // This can be interpreted as the time between game ticks.
    polling_interval_ms: usize,
}

impl SnakeGame {

    /// Construct a new SnakeGame object.
    pub fn new() -> Self {
        let egg = Dot { x: EGG_START_X, y: EGG_START_Y};
        let snake = Snake::new();
        let screen = DotScreen::new_empty();
        let mut game = Self { egg, snake, screen, polling_interval_ms: INITIAL_POLL_INTERVAL };
        game.reset();
        return game
    }

    /// Returns the current score for the game.
    pub fn get_score(&self) -> usize {
        self.snake.get_length() - START_LENGTH
    }

    /// Update the game state.
    /// 
    /// This is called for every game tick. This function will move the Snake
    ///   in the direction its Head is pointing, and then resolves the games state.
    /// 
    /// # Returns 
    /// Whether the game state was successfully updated.
    fn update(&mut self) -> bool {
        match self.snake.slither(&self.egg) {
            SlitherResult::Moved(dropped_segment) => {
                self.screen.remove(&dropped_segment.position);
                self.screen.add(&self.snake.head.position);
            },
            SlitherResult::EggEaten => {
                // Place a new egg in an open dot.
                let index = self.get_random_index();
                self.egg = self.screen.iter_off().nth(index).unwrap();
                self.screen.add(&self.egg);

                // Decrease the time between game ticks.
                self.increase_speed();
            },
            SlitherResult::Collision => {
                // If a collision occurred, then the game did not successfully update.
                return false
            }
        }
        return true
    }

    /// This function uses the current state of the snake to generate a random
    ///   number between 0 and the number of unlit dots on the screen.
    fn get_random_index(&mut self) -> usize {
        let last_dot = self.snake.tail.back().unwrap().position;
        let modulus = DotScreen::TOTAL_DOTS - self.snake.get_length();
        ((last_dot.x << 4) | last_dot.y) % modulus
    }

    /// Decrease the time between game ticks.
    fn increase_speed(&mut self) {
        self.polling_interval_ms -= self.polling_interval_ms / 50;
    }
}

impl Game for SnakeGame {

    /// This method is called when the game is over.
    /// 
    /// When the game over state is complete, this method should return.
    /// 
    /// # Args
    /// * components - The peripheral components for the game display.
    fn game_over(&self, components: &mut Components) {
        // Flash between the last game state screen and an empty screen,
        //   to indicate that the player has lost the game.
        let mut game_over_screen = DotScreen::new_empty();
        components.display.show(&game_over_screen);
        for _ in 0..2 {
            arduino_uno::delay_ms(400);
            components.display.show(&self.screen);
            arduino_uno::delay_ms(400);
            components.display.show(&game_over_screen);
        }
        
        // Display the game score to the user by displaying a dot for each egg eaten,
        //   one at a time, from left to right, top to bottom of the screen.
        let display = &mut components.display;
        let delay = 3000 / (self.get_score() as u16);
        DotScreen::new_empty()
            .iter()
            .take(self.get_score())
            .for_each(|dot| {
                game_over_screen.add(&dot);
                display.show(&game_over_screen);
                arduino_uno::delay_ms(delay);
            }
        );
        
        // Loop waiting for a JoyStick button press to end the game over screen.
        loop {
            match components.joystick.poll_until_any() {
                InputSignal::JoyStick(signal) => {
                    if let JoyStickSignal { button: true, .. } = signal { break }
                }
            }
        }
    }

    /// This method is called to begin the game-play.
    /// 
    /// This is expected to construct its own game loop. Once the game-play
    ///   ends, this method should return.
    /// 
    /// # Args
    /// * components - The peripheral components for the game display.
    fn play(&mut self, components: &mut Components) {
        loop {
            // Gather user input, for the amount of milliseconds stored in the 
            //   `self.polling_interval_ms` attribute.
            // This interval gets shorter and shorter as more eggs are eaten,
            //   increasing the difficulty of the game.
            let signal = 
                components.joystick.poll(self.polling_interval_ms).back();
            if let Some(InputSignal::JoyStick(signal)) = signal {
                if let Some(direction) = signal.to_single_direction() {
                    self.snake.set_direction(direction);
                };
            };

            // Update the game state. If unsuccessful, break out the game loop.
            let update_successful = self.update();
            if !update_successful { break }

            // Display the game state to the LED Dot Display.
            components.display.show(&self.screen);
        }
    }

    /// This method is called to reset the game to its initial state.
    /// 
    /// After this method is called, the game should be ready to be played again.
    fn reset(&mut self) {
        // Reset the Egg.
        self.egg = Dot { x: EGG_START_X, y: EGG_START_Y};

        // Reset the Snake.
        self.snake.init();

        // Clear and reset the Screen.
        self.screen.clear();
        self.screen.add(&self.egg);
        self.screen.add(&self.snake.head.position);
        for segment in self.snake.tail.iter() {
            self.screen.add(&segment.position)
        }

        // Reset the polling interval.
        self.polling_interval_ms = INITIAL_POLL_INTERVAL;
    }

    /// This method returns the title screen for the game.
    /// 
    /// This method is non-static so that this trait can become a trait object.
    /// 
    /// # Returns
    /// The DotScreen object which displays as the title screen.
    fn title_screen(&self) -> DotScreen {
        DotScreen::new(
            [
                0b00000000,
                0b00000000,
                0b11011111,
                0b10011001,
                0b10011001,
                0b11111011,
                0b00000000,
                0b00000000,
            ]
        )
    }
}