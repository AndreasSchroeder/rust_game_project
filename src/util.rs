use SIZE_PER_TILE;
use BORDER_BETWEEN_TILES;

pub fn coord_to_pixel_x(world: u64, cam: u64) -> f64 {
    ((world - cam) * (SIZE_PER_TILE * BORDER_BETWEEN_TILES)) as f64
}
pub fn coord_to_pixel_y(world: u64, cam: u64) -> f64 {
    ((world - cam) * (SIZE_PER_TILE * BORDER_BETWEEN_TILES)) as f64
}