use std::process::exit;

/* -------------------------------------------------------------------------- */
/*                                 Grid System                                */
/* -------------------------------------------------------------------------- */
use crate::collision_system::{bbox_intersect, point_intersect};
use crate::{Particles, AABB};
use macroquad::math::Vec2;

#[derive(Clone, Debug)]
pub struct Tile {
    pub index: usize,
    pub bounding_box: AABB,
    // num_particles: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Grid {
    // width: f32,
    // height: f32,
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: f32, height: f32, n_x: usize, n_y: usize) -> Self {
        let mut tiles = Vec::new();
        for num_x in 0..n_x {
            for num_y in 0..n_y {
                tiles.push(Tile {
                    index: num_x * n_y + num_y,
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
                    // num_particles: None,
                });
            }
        }
        return Self {
            // width,
            // height,
            tiles,
        };
    }

    pub fn update_paricle_locations_in_grid(self: &Self, particles: &Particles) -> Particles {
        // The new particles
        let mut new_particles: Particles = Vec::new();

        // Store old particles for comparison and popping
        let old_particles: Particles = particles.clone();
        let mut working_particles: Particles = particles.clone();

        while let Some(particle) = working_particles.pop() {
            // println!("---------- PARTICLE ------------");
            // dbg!(particle.pos);
            let mut working_tiles: Vec<Tile> = self.tiles.clone();
            while let Some(tile) = working_tiles.pop() {
                if point_intersect(&particle, &tile.bounding_box) {
                    let mut clone = particle.clone();
                    clone.tile = Some(tile.index);
                    // println!("----------- found the tile! ------------");
                    // dbg!(tile.index);
                    // dbg!(tile.bounding_box);
                    new_particles.push(clone);
                    // break;
                }
            }
        }
        if new_particles.len() != particles.len() {
            dbg!(new_particles.len());
            dbg!(old_particles.len());
            let e = 1;
            println!("{}", e);
            exit(e);
        }
        return new_particles;
    }
}
// Hey! I have a grid for the windows with indeces and bounding boxes.
// If you give me a list of particles, I'll tell you which ones are in the same grid square so you can check them for collissions.
// I just need to borrow the particles, I won't be changing them.
