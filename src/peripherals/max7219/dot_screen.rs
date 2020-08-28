use super::Dot;


/// The DotScreen is the object used to create scene on the DotDisplay,
///   that is passed to the DotDisplay::show function.
#[derive(Copy, Clone)]
pub struct DotScreen {
    /// The binary representation of these 8-bit unsigned integers encode
    ///   the on-off state of each LED within the column of the DotDisplay.
    /// The MSB is the top of the column, and the columns are ordered left to right.
    pub columns: [u8; 8],
}

impl DotScreen {

    // The constants describing the dimensions of the DotScreen.
    pub const HEIGHT: usize = 8;
    pub const WIDTH: usize = 8;
    pub const TOTAL_DOTS: usize = Self::HEIGHT * Self::WIDTH;

    /// Creates a new DotScreen object, from the columns provided.
    pub const fn new(columns: [u8; 8]) -> Self {
        DotScreen { columns }
    }

    /// Creates a new DotScreen object, with all LEDs turned off.
    pub const fn new_empty() -> Self {
        DotScreen { columns: [0u8; 8] }
    }

    /// Creates a new DotScreen object, with all LEDs turned on.
    pub const fn new_full() -> Self {
        DotScreen { columns: [255u8; 8] }
    }

    /// Add a dot to the DotScreen.
    /// 
    /// This will turn on the LED, this does not toggle the LED.
    pub fn add(&mut self, dot: &Dot) {
        self.columns[dot.x] |= 1 << (7 - dot.y);
    }

    /// Turn off all the LEDs on the DotScreen.
    pub fn clear(&mut self) {
        for column in self.columns.iter_mut() {
            *column &= 0;
        }
    }
    
    /// Remove a dot to the DotScreen.
    /// 
    /// This will turn off the LED, this does not toggle the LED.
    pub fn remove(&mut self, dot: &Dot) {
        self.columns[dot.x] &= !(1 << (7 - dot.y));
    }

    /// Helper function used to determine if a particular dot LED is turned on.
    #[inline(always)]
    pub fn is_dot_on(&self, dot: &Dot) -> bool {
        self.is_on(dot.x, dot.y)
    }

    /// Helper function used to determine if a particular dot LED is turned off.
    #[inline(always)]
    pub fn is_dot_off(&self, dot: &Dot) -> bool {
        self.is_off(dot.x, dot.y)
    }

    /// Helper function used to determine if the LED dot at the specified
    ///   x, y position is turned on.
    #[inline(always)]
    pub fn is_on(&self, x: usize, y: usize) -> bool {
        (self.columns[x] & (1 << (7 - y))) != 0
    }

    /// Helper function used to determine if the LED dot at the specified
    ///   x, y position is turned off.
    #[inline(always)]
    pub fn is_off(&self, x: usize, y: usize) -> bool {
        !self.is_on(x, y)
    }

    /// Helper function that acts as the identity function for an IterDotScreen object.
    #[inline(always)]
    fn is_dot(&self, x: usize, y: usize) -> bool {
        (x < Self::WIDTH) & (y < Self::HEIGHT)
    }

    /// Create an iterator object over all dots of the DotScreen.
    pub fn iter<'d>(&'d self) -> IterDotScreen<'d> {
        IterDotScreen::new(&self, &Self::is_dot)
    }

    /// Create an iterator object over all dots of the DotScreen that are on.
    pub fn iter_on<'d>(&'d self) -> IterDotScreen<'d> {
        IterDotScreen::new(&self, &Self::is_on)
    }

    /// Create an iterator object over all dots of the DotScreen that are off.
    pub fn iter_off<'d>(&'d self) -> IterDotScreen<'d> {
        IterDotScreen::new(&self, &Self::is_off)
    }
}


type DotScreenPredicate<'d> = dyn Fn(&'d DotScreen, usize, usize) -> bool;

/// Iterator over a DotScreen.
pub struct IterDotScreen<'d> {
    // The DotScreen object being iterated over.
    inner: &'d DotScreen,
    // The predicate function used to determine whether to yield a particular Dot.
    func: &'d DotScreenPredicate<'d>,
    // The current x-coordinate of the iterator.
    x: usize,
    // The current y-coordinate of the iterator.
    y: usize,
}

impl<'d> IterDotScreen<'d> {
    fn new(inner: &'d DotScreen, func: &'d DotScreenPredicate<'d>) -> Self {
        Self { inner, func, x: 0, y: DotScreen::HEIGHT - 1}
    }
}

impl<'d> Iterator for IterDotScreen<'d> {   
    type Item = Dot;
    fn next(&mut self) -> Option<Self::Item> {
        for y in (0..=self.y).rev() {
            for x in self.x..DotScreen::WIDTH {
                if (self.func)(self.inner, x, y) {
                    self.x = x + 1;
                    self.y = y;
                    return Some(Dot { x, y })
                }
            }
            self.x = 0;
        }
        None
    }
}