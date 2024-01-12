use macroquad::window::{screen_height, screen_width};

use crate::{
    collision_system::handle_wall_collisions,
    grid::{update_tiles, Grid},
    Particle,
};

pub fn update(dt: &f32, old_particles: Vec<Particle>) -> Vec<Particle> {
    // clone list to get new particle list and return that

    let mut new_particles: Vec<Particle> = old_particles.clone();

    // calculate new positions
    for new_particle in new_particles.iter_mut() {
        new_particle.vel.x += new_particle.acc.x * dt;
        new_particle.vel.y += new_particle.acc.y * dt;
        new_particle.pos.x += new_particle.vel.x * dt;
        new_particle.pos.y += new_particle.vel.y * dt;
        new_particle.acc = new_particle.acc;
    }

    // check if any hit the walls and adjust
    for (new_particle, old_particle) in new_particles.iter_mut().zip(old_particles.iter()) {
        handle_wall_collisions(new_particle, old_particle)
    }

    let grid: Grid = Grid::new(screen_width(), screen_height(), 9, 9);
    let grid: Grid = update_tiles(&new_particles, grid);

    // let presence: Vec<usize> = (&new_particles, grid);

    return new_particles;
}
