pub mod grid;
pub mod token;

use cgmath::Vector2;

fn long_radius(short_radius: f32) -> f32 {
    short_radius * 2f32 / 3f32.sqrt()
}
fn short_radius(long_radius: f32) -> f32 {
    long_radius * 3f32.sqrt() / 2f32
}

fn grid_to_world(coords: Vector2<u32>, tile_size: f32) -> Vector2<f32> {
    let stepx = short_radius(tile_size);
    let stepy = tile_size * 3.0 / 4.0;

    let x = coords.x as f32 * stepx - if coords.y % 2 == 0 { stepx / 2.0 } else { 0.0 };
    let y = coords.y as f32 * stepy;
    Vector2::new(x, y)
}

fn grid_coords(height: u32, width: u32, tile_size: f32) -> Vec<f32> {
    let stepx = short_radius(tile_size);
    let stepy = tile_size * 3.0 / 4.0;

    let mut grid = Vec::new();
    for row in 0..height {
        for i in 0..width {
            grid.push(i as f32 * stepx - if row % 2 == 0 { stepx / 2.0 } else { 0.0 });
            grid.push(row as f32 * stepy);
        }
    }
    grid
}

fn corner_offset(tile_size: f32, corner: u8) -> Vector2<f32> {
    assert!(corner < 6);
    match corner {
        0 => Vector2::new(short_radius(tile_size), 2.0 * tile_size),
        1 => Vector2::new(2.0 * short_radius(tile_size), 3.0 / 4.0 * tile_size),
        2 => Vector2::new(2.0 * short_radius(tile_size), 1.0 / 4.0 * tile_size),
        3 => Vector2::new(short_radius(tile_size), 0.0),
        4 => Vector2::new(0.0, 1.0 / 4.0 * tile_size),
        5 => Vector2::new(0.0, 3.0 / 4.0 * tile_size),
        _ => unreachable!()
    }

}
