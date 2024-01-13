use macroquad::window::{screen_height, screen_width};

use crate::{collision_system::handle_wall_collisions, grid::Grid, Particles};

pub fn update(dt: &f32, old_particles: Particles) -> Particles {
    // clone list to get new particle list and return that

    let mut new_particles = old_particles.clone();

    for i in 0..new_particles.number {
        new_particles.list.entry(i).and_modify(|p| {
            p.vel.x += p.acc.x * dt;
            p.vel.y += p.acc.y * dt;
            p.pos.x += p.vel.x * dt;
            p.pos.y += p.vel.y * dt;
            p.acc = p.acc;
        });
    }
    // new_particles.entry(0);
    // calculate new positions
    // for new_particle in new_particles.iter_mut() {
    //     new_particle.vel.x += new_particle.acc.x * dt;
    //     new_particle.vel.y += new_particle.acc.y * dt;
    //     new_particle.pos.x += new_particle.vel.x * dt;
    //     new_particle.pos.y += new_particle.vel.y * dt;
    //     new_particle.acc = new_particle.acc;
    // }

    // check if any hit the walls and adjust
    for (new_particle, old_particle) in new_particles.iter_mut().zip(old_particles.list.iter()) {
        handle_wall_collisions(new_particle, old_particle)
    }

    let grid: Grid = Grid::new(screen_width(), screen_height(), 9, 9);
    grid.update_tiles(&new_particles);

    for tile in grid.tiles.iter() {
        let particle_list = &tile.particles;
        if let Some(particle_list) = particle_list {}
    }
    // let presence: Vec<usize> = (&new_particles, grid);

    return new_particles;
}
