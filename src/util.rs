use SIZE_PER_TILE;
use BORDER_BETWEEN_TILES;
use CAM_BORDER;
use HUB_UP;

pub fn coord_to_pixel_x(world: u64, cam: u64) -> f64 {
    (((world - cam) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES)) + CAM_BORDER ) as f64
}
pub fn coord_to_pixel_y(world: u64, cam: u64) -> f64 {

    ((world - cam) * (SIZE_PER_TILE + BORDER_BETWEEN_TILES) + HUB_UP) as f64
}