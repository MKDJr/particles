/* -------------------------------------------------------------------------- */
/*                                 Grid System                                */
/* -------------------------------------------------------------------------- */

use crate::collision_system::intersect;
use crate::{Particle, AABB};
use macroquad::math::Vec2;

pub struct Grid {
    tiles: Vec<Tile>,
}

#[derive(Clone)]
struct Tile {
    x_index: u32,
    y_index: u32,
    bounding_box: AABB,
    particles: Option<Vec<Particle>>,
}

impl Grid {
    pub fn new(width: f32, height: f32, n_x: u32, n_y: u32) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for num_x in 0..n_x {
            for num_y in 0..n_y {
                tiles.push(Tile {
                    x_index: num_x,
                    y_index: num_y,
                    bounding_box: AABB {
                        lower_bound: Vec2 {
                            x: (width * num_x as f32 / n_x as f32),
                            y: (height * num_y as f32 / n_y as f32),
                        },
                        upper_bound: Vec2 {
                            x: (width * (num_x as f32 + 1.) / n_x as f32),
                            y: (height * (num_y as f32 + 1.) / n_y as f32),
                        },
                    },
                    particles: None,
                });
            }
        }
        return Self { tiles };
    }
}

fn update_tiles(particles: &Vec<Particle>, grid: Grid) -> Grid {
    let mut particles: Vec<Particle> = particles.clone();
    let mut grid: Grid = grid;

    let mut updated_grid: Grid = Grid { tiles: Vec::new() };

    while let Some(tile) = grid.tiles.pop() {
        let mut updated_tile: Tile = tile.clone();

        while let Some(particle) = particles.pop() {
            let mut current_particles_list: Vec<Particle> = Vec::new();

            if intersect(particle.bounding_box, tile.bounding_box) {
                current_particles_list.push(particle)
            }
            updated_tile.particles = Some(current_particles_list);
        }
        updated_grid.tiles.push(updated_tile)
    }
    return updated_grid;
}

// Hey! I have a grid for the windows with indeces and bounding boxes.
// If you give me a list of particles, I'll tell you which ones are in the same grid square so you can check them for collissions.
// I just need to borrow the particles, I won't be changing them.
