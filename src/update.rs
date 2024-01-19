use crate::{
    collision_system::{handle_particle_collision, handle_wall_collisions, intersect},
    grid::{draw_bb, Grid},
    Particle, Particles,
};
use macroquad::{
    color::BLUE,
    math::Vec2,
    window::{screen_height, screen_width},
};
use std::process::exit;

pub fn update_world(dt: &f32, frozen_particles: Particles) -> Particles {
    // This function updates the world state by `dt` time.
    // It takes ownership of the frozen particle state.
    // It returns a new set of particles to `main` which have been updated to the next tick.
    // Every function after it takes the previous output as input.

    // dbg!(&frozen_particles.len());

    // Update the particles position based on their velocity and their velocity based on their acceleration.
    let eulerian_updated_particle: Particles = euler_update(frozen_particles.clone(), dt);

    // dbg!(&eulerian_updated_particle.len());

    // Check if particles collide with the wall and handle.
    let walled_particles: Particles = update_particles_for_wall_conditions(
        eulerian_updated_particle.clone(),
        frozen_particles.clone(),
    );

    // dbg!(&walled_particles.len());

    let grid: Grid = Grid::new(screen_width(), screen_height(), 3, 3);
    for tile in grid.tiles.clone().drain(..) {
        draw_bb(tile.bounding_box, BLUE);
    }

    let gridded_particles: Particles = grid.update_paricle_locations_in_grid(&walled_particles);
    // dbg!(&gridded_particles.len());

    // Drain a clone of the tiles
    let new_particles: Particles = update_particles_for_particle_collision(gridded_particles);
    // dbg!(&new_particles.len());

    return new_particles;
}

fn euler_update(mut input_particles: Particles, dt: &f32) -> Particles {
    let mut output_particles = Particles::new();
    for particle in input_particles.drain(..) {
        output_particles.push(Particle::new(
            particle.pos + particle.vel * Vec2::splat(dt.clone()),
            particle.vel + particle.acc * Vec2::splat(dt.clone()),
            particle.acc,
            particle.mass,
        ));
    }
    return output_particles;
}

fn update_particles_for_wall_conditions(
    mut eulerian_updated_particles: Particles,
    mut frozen_particles: Particles,
) -> Particles {
    let temp = eulerian_updated_particles.clone();
    let mut walled_particles: Particles = Particles::new();

    for (eul_particle, old_particle) in eulerian_updated_particles
        .drain(..)
        .zip(frozen_particles.drain(..))
    {
        walled_particles.push(handle_wall_collisions(&eul_particle, &old_particle));
    }
    if walled_particles.len() != temp.len() {
        dbg!(temp.len());
        dbg!(walled_particles.len());

        let e = 2;
        println!("{}", e);
        exit(e);
    }
    return walled_particles;
}

fn update_particles_for_particle_collision(old_particles: Particles) -> Particles {
    let mut new_particles: Particles = Particles::new();

    if old_particles.len() == 1 {
        new_particles.push(old_particles[0]);
        return new_particles;
    }

    let mut particles_grouped_by_tile: Vec<Particles> = group(&old_particles);
    while let Some(mut working_particles) = particles_grouped_by_tile.pop() {
        dbg!(working_particles.len());

        // of particles in a tile, select one
        while let Some(particle) = working_particles.pop() {
            let mut others: Particles = working_particles.clone();
            dbg!(others.len());

            // other tiles not covered
            while let Some(other) = others.pop() {
                // Check if they intersect
                if dbg!(intersect(particle.bounding_box, other.bounding_box)) {
                    // If so, calculate their new attributes
                    let (new_a_vel, new_b_vel) = handle_particle_collision(&particle, &other);
                    let mut a = particle.clone();
                    a.vel = new_a_vel;
                    new_particles.push(a);

                    let mut b = other.clone();
                    b.vel = new_b_vel;
                    new_particles.push(b);

                    working_particles.retain(|&x| x != b); // find b and pop
                    break;
                }
            }
        }
    }
    if new_particles.len() != old_particles.len() {
        dbg!(new_particles.len());
        dbg!(old_particles.len());
        let e = 3;
        println!("{}", e);
        exit(e);
    }
    return new_particles;
}

fn combos(particles: &Particles) -> Vec<Particles> {
    let mut working_particles: Particles = particles.clone();
    let mut combos: Vec<Particles> = Vec::new();

    // dbg!(working_particles.len());
    while let Some(a) = working_particles.pop() {
        // dbg!(a);
        let mut temp_particles: Particles = working_particles.clone();
        // dbg!(temp_particles.len());
        while let Some(b) = temp_particles.pop() {
            // dbg!(b);
            let mut temp: Particles = Particles::new();
            temp.push(a);
            temp.push(b);
            combos.push(temp);
        }
    }
    {
        let len = particles.len();
        // dbg!(len);
        let num = len * (len - 1) / 2;
        // dbg!(num);
        if combos.len() != num {
            let e = 4;
            println!("{}", e);
            exit(e);
        };
    }
    return combos;
}

fn group(particles: &Particles) -> Vec<Particles> {
    let mut sorted_particles: Particles = particles.clone();

    sorted_particles.sort_by(|a, b| a.tile.cmp(&b.tile));
    let mut particles_grouped_by_tile: Vec<Particles> = Vec::new();
    let mut temp = None;

    while let Some(particle) = sorted_particles.pop() {
        match particle.tile {
            // If the particle has a tile
            Some(t) => match temp {
                // and the counter indicating which tile we're on exists
                Some(temp_work) => {
                    // and they're not equal, add new vector to the main vector, set tile counter
                    if t != temp_work {
                        particles_grouped_by_tile.push(vec![particle]);
                        temp = Some(t);

                    // but if they are equal, push the current particle to the last vector in the main vector
                    } else if let Some(last) = particles_grouped_by_tile.last_mut() {
                        last.push(particle);
                    }
                }

                // but if the counter doesn't exist, you are on cycle one so set it and add the particle as a new vector
                None => {
                    temp = Some(t);
                    particles_grouped_by_tile.push(vec![particle]);
                }
            },

            // If the particle doesn't have a tile, skip
            None => {}
        }
    }

    return particles_grouped_by_tile;
}
