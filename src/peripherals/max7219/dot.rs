use super::DotScreen;

/// LED on the DotDisplay.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Dot {
    pub x: usize,
    pub y: usize,
}

impl Dot {

    /// Returns the Dot Left of the current dot.
    /// 
    /// If the current dot is at the Left edge of the screen,
    ///   this returns the current dot. This simulates "hitting"
    ///   the wall.
    pub fn left(&self) -> Self { 
        let x = 
            if self.x > 0 { 
                self.x - 1 
            } else { 
                self.x 
            };
        Self { x, y: self.y }
    }

    /// Returns the Dot Right of the current dot.
    /// 
    /// If the current dot is at the Right edge of the screen,
    ///   this returns the current dot. This simulates "hitting"
    ///   the wall.
    pub fn right(&self) -> Self {
        let x = 
            if self.x < DotScreen::WIDTH - 1 { 
                self.x + 1 
            } else { 
                self.x 
            };
        Self { x, y: self.y }
    }

    /// Returns the Dot Above the current dot.
    /// 
    /// If the current dot is at the Top edge of the screen,
    ///   this returns the current dot. This simulates "hitting"
    ///   the wall.
    pub fn up(&self) -> Self { 
        let y = 
            if self.y < DotScreen::HEIGHT - 1 { 
                self.y + 1 
            } else { 
                self.y 
            };
        Self { x: self.x, y }
    }

    /// Returns the Dot Below the current dot.
    /// 
    /// If the current dot is at the Bottom edge of the screen,
    ///   this returns the current dot. This simulates "hitting"
    ///   the wall.
    pub fn down(&self) -> Self {
        let y = 
            if self.y > 0 { 
                self.y - 1 
            } else { 
                self.y 
            };
        Self { x: self.x, y }
    }

    /// Moves the current Dot to the Left.
    /// 
    /// If the current dot is at the Left edge of the screen,
    ///   the dot remains unchanged. This simulates "hitting"
    ///   the wall.
    pub fn move_left(&mut self) { 
        if self.x > 0 {
            self.x -= 1;
        } 
    }

    /// Moves the current Dot to the Right.
    /// 
    /// If the current dot is at the Right edge of the screen,
    ///   the dot remains unchanged. This simulates "hitting"
    ///   the wall.
    pub fn move_right(&mut self) { 
        if self.x < DotScreen::WIDTH - 1 { 
            self.x += 1; 
        } 
    }

    /// Moves the current Dot Upward.
    /// 
    /// If the current dot is at the Top edge of the screen,
    ///   the dot remains unchanged. This simulates "hitting"
    ///   the wall.
    pub fn move_up(&mut self) { 
        if self.y < DotScreen::HEIGHT - 1 { 
            self.y += 1; 
        } 
    }

    /// Moves the current Dot Downward.
    /// 
    /// If the current dot is at the Bottom edge of the screen,
    ///   the dot remains unchanged. This simulates "hitting"
    ///   the wall.
    pub fn move_down(&mut self) { 
        if self.y > 0 { 
            self.y -= 1; 
        } 
    }
}