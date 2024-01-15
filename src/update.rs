use core::panic;

use crate::{
    collision_system::{handle_particle_collision, handle_wall_collisions, intersect},
    grid::Grid,
    Particle, Particles,
};
use itertools::Itertools;
use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

pub fn update_world(dt: &f32, frozen_particles: Particles) -> Particles {
    // This function updates the world state by `dt` time.
    // It takes ownership of the frozen particle state.
    // It returns a new set of particles to `main` which have been updated to the next tick.
    // Every function after it takes the previous output as input.

    // Update the particles position based on their velocity and their velocity based on their acceleration.
    let eulerian_updated_particle = euler_update(frozen_particles.clone(), dt);

    // Check if particles collide with the wall and handle.
    let mut walled_particles = update_particles_for_wall_conditions(
        eulerian_updated_particle.clone(),
        frozen_particles.clone(),
    );

    let mut grid: Grid = Grid::new(screen_width(), screen_height(), 9, 9);

    grid = grid.update_tiles(&walled_particles);

    // Drain a clone of the tiles
    for (idx, tile) in grid.tiles.clone().drain() {
        // Eat up the tile and make an iterator through the idxs
        let work = tile.particle_idxs.iter();

        // Create all possible pairs of colliding particles
        let particle_idx_combinations: itertools::Combinations<std::slice::Iter<'_, usize>> =
            work.combinations(2);

        // Grab particles from idxs
        for combo in particle_idx_combinations {
            let Some(a) = walled_particles.list.get(combo[0]) else {
                panic!("help")
            };
            let Some(b) = walled_particles.list.get(combo[1]) else {
                panic!()
            };

            // Check if they intersect
            if intersect(a.bounding_box, b.bounding_box) {
                // If so, calculate their new attributes
                let (new_a_vel, new_b_vel) = handle_particle_collision(a, b);
                walled_particles
                    .list
                    .entry(*combo[0])
                    .and_modify(|p| p.vel = new_a_vel);
                walled_particles
                    .list
                    .entry(*combo[1])
                    .and_modify(|p| p.vel = new_b_vel);
            }
        }
    }

    return walled_particles;
}

fn euler_update(mut input_particles: Particles, dt: &f32) -> Particles {
    let mut output_particles = Particles::new();
    for (idx, particle) in input_particles.list.drain() {
        output_particles.list.insert(
            idx,
            Particle::new(
                particle.pos + particle.vel * Vec2::splat(dt.clone()),
                particle.vel + particle.acc * Vec2::splat(dt.clone()),
                particle.acc,
                particle.mass,
            ),
        );
    }

    return output_particles;
}

fn update_particles_for_wall_conditions(
    mut eulerian_updated_particles: Particles,
    frozen_particles: Particles,
) -> Particles {
    //
    let mut walled_particles = Particles::new();
    for (idx, eul_particle) in eulerian_updated_particles.list.drain() {
        walled_particles.list.insert(
            idx,
            handle_wall_collisions(&eul_particle, &frozen_particles.list[&idx]),
        );
    }
    return walled_particles;
}
