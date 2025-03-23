// Constants for the pixel grid
pub const GRID_WIDTH: usize = 768; // Horizontal pixels
pub const GRID_HEIGHT: usize = 480; // Vertical pixels

// Get the size of the screen in world units
pub fn get_screen_size() -> (f32, f32) {
    (
        GRID_WIDTH as f32,
        GRID_HEIGHT as f32,
    )
}

// Get half the width and height of the screen in world units
pub fn get_half_screen_size() -> (f32, f32) {
    let (width, height) = get_screen_size();
    (width / 2.0, height / 2.0)
}
