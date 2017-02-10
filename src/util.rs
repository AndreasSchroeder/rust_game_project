// Module for Utilities
use SIZE_PER_TILE;
use BORDER_BETWEEN_TILES;
use CAM_BORDER;
use HUB_UP;
/// Calculate the pixel Coordinates from given world and Camera Coordinates for x
/// Take const froms Main to set tile-width and the amount of pixel between Tiles and
/// calculates from them with the camera-border (left) the pixel coordinates of the window.
pub fn coord_to_pixel_x(world: u64, cam: u64) -> f64 {
    (((world - cam) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) + CAM_BORDER) as f64
}
/// Calculate the pixel Coordinates from given world and Camera Coordinates for y
/// Take const froms Main to set tile-height and the amount of pixel between Tiles and
/// calculates from them with the camera-border (left) the pixel coordinates of the window.
pub fn coord_to_pixel_y(world: u64, cam: u64) -> f64 {

    ((world - cam) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES) + HUB_UP) as f64
}
