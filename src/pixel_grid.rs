// Constants for the pixel grid
pub const GRID_WIDTH: usize = 32; // Horizontal pixels
pub const GRID_HEIGHT: usize = 24; // Vertical pixels
pub const PIXEL_SIZE: f32 = 25.0; // Size of each pixel in world units

// Get the size of the screen in world units
pub fn get_screen_size() -> (f32, f32) {
    (
        GRID_WIDTH as f32 * PIXEL_SIZE,
        GRID_HEIGHT as f32 * PIXEL_SIZE
    )
}

// Get half the width and height of the screen in world units
pub fn get_half_screen_size() -> (f32, f32) {
    let (width, height) = get_screen_size();
    (width / 2.0, height / 2.0)
}
