use macroquad::math::Vec2;
use crate::{screen_height, screen_width, Particle, AABB, COEF_OF_RESTITUTION, RADIUS};

pub fn handle_wall_collisions(new_particle: &mut Particle, old_particle: &Particle) {
    if new_particle.pos.y - RADIUS < 0. {
        new_particle.vel.y = -new_particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (0. + RADIUS - old_particle.pos.y) / (new_particle.pos.y - old_particle.pos.y);

        new_particle.pos.y = old_particle.pos.y
            + fraction_of_trajectory_before_collision * (new_particle.pos.y - old_particle.pos.y)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.y - old_particle.pos.y);
    }
    if new_particle.pos.y + RADIUS > screen_height() {
        new_particle.vel.y = -new_particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_height() - RADIUS - old_particle.pos.y)
                / (new_particle.pos.y - old_particle.pos.y);

        new_particle.pos.y = old_particle.pos.y
            + fraction_of_trajectory_before_collision * (new_particle.pos.y - old_particle.pos.y)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.y - old_particle.pos.y);
    }
    if new_particle.pos.x - RADIUS < 0. {
        new_particle.vel.x = -new_particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (0. + RADIUS - old_particle.pos.x) / (new_particle.pos.x - old_particle.pos.x);

        new_particle.pos.x = old_particle.pos.x
            + fraction_of_trajectory_before_collision * (new_particle.pos.x - old_particle.pos.x)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.x - old_particle.pos.x);
    }
    if new_particle.pos.x + RADIUS > screen_width() {
        new_particle.vel.x = -new_particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_width() - RADIUS - old_particle.pos.x)
                / (new_particle.pos.x - old_particle.pos.x);
        new_particle.pos.x = old_particle.pos.x
            + fraction_of_trajectory_before_collision * (new_particle.pos.x - old_particle.pos.x)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.x - old_particle.pos.x);
    }
}

// Return "true" if two bounding boxes intersect, return "false" otherwise.
pub fn intersect(a: AABB, b: AABB) -> bool {
    if a.lower_bound.x < b.upper_bound.x {
        return true;
    } else if a.upper_bound.x > b.lower_bound.x {
        return true;
    } else if a.lower_bound.y < b.upper_bound.y {
        return true;
    } else if a.upper_bound.y > b.lower_bound.y {
        return true;
    } else {
        return false;
    }
}

pub fn handle_particle_collision(a: &Particle, b: &Particle) -> (Vec2, Vec2) {
    let collision_vector = Vec2 {
        x: (b.pos.x - a.pos.x),
        y: (b.pos.y - a.pos.y),
    };

    let new_a_vel = a.vel
        - (2. * b.mass / (a.mass + b.mass))
            * (((a.vel - b.vel).dot(a.pos - b.pos)) / (a.pos - b.pos).length_squared())
            * (a.pos - b.pos);

    let new_b_vel = b.vel
        - (2. * a.mass / (a.mass + b.mass))
            * (((b.vel - a.vel).dot(b.pos - a.pos)) / (b.pos - a.pos).length_squared())
            * (b.pos - a.pos);

    // a.mass * a.vel1 + b.mass * b.vel1 = a.mass * a.vel2 + b.mass * b.vel2
    // a.vel2 = ((a.mass * a.vel1 + b.mass * b.vel1) - (b.mass * b.vel2)) / a.mass
    // b.vel2 = ((a.mass * a.vel1 + b.mass * b.vel1) - (a.mass * a.vel2)) / b.mass
    // a.vel2 = ((a.mass * a.vel1 + b.mass * b.vel1) - (b.mass * (((a.mass * a.vel1 + b.mass * b.vel1) - (a.mass * a.vel2)) / b.mass)))) / a.mass

    return (new_a_vel, new_b_vel);
}
