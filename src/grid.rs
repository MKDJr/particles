use std::collections::HashMap;

/* -------------------------------------------------------------------------- */
/*                                 Grid System                                */
/* -------------------------------------------------------------------------- */
use crate::collision_system::intersect;
use crate::{Particles, AABB};
use macroquad::math::Vec2;

#[derive(Clone, Debug)]
pub struct Tile {
    bounding_box: AABB,
    pub particle_idxs: Vec<usize>,
}
#[derive(Clone, Debug)]
pub struct Grid {
    width: f32,
    height: f32,
    pub tiles: HashMap<usize, Tile>,
}

impl Grid {
    pub fn new(width: f32, height: f32, n_x: usize, n_y: usize) -> Self {
        let mut tiles = HashMap::new();
        for num_x in 0..n_x {
            for num_y in 0..n_y {
                tiles.insert(
                    num_x * n_y + num_y,
                    Tile {
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
                        particle_idxs: Vec::new(),
                    },
                );
            }
        }
        return Self {
            width,
            height,
            tiles,
        };
    }

    pub fn update_tiles(self: &Self, particles: &Particles) -> Self {
        let mut working_particle_list = particles.list.clone();
        let mut working_tiles = self.tiles.clone();

        for (idx, particle) in working_particle_list.drain() {
            for k in 0..working_tiles.len() {
                working_tiles.entry(k).and_modify(|tile| {
                    if intersect(particle.bounding_box, tile.bounding_box) {
                        tile.particle_idxs.push(idx);
                    }
                });
            }
        }
        return Grid {
            width: self.width,
            height: self.height,
            tiles: working_tiles,
        };
    }
}
// Hey! I have a grid for the windows with indeces and bounding boxes.
// If you give me a list of particles, I'll tell you which ones are in the same grid square so you can check them for collissions.
// I just need to borrow the particles, I won't be changing them.
