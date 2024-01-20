use crate::{screen_height, screen_width, Particle, AABB, COEF_OF_RESTITUTION};
use macroquad::math::Vec2;

pub fn handle_wall_collisions(curr_particle: &Particle, old_particle: &Particle) -> Particle {
    let mut new_particle = curr_particle.clone();
    if new_particle.pos.y - old_particle.radius < 0. {
        new_particle.vel.y = -new_particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision = (0. + old_particle.radius
            - old_particle.pos.y)
            / (new_particle.pos.y - old_particle.pos.y);

        new_particle.pos.y = old_particle.pos.y
            + fraction_of_trajectory_before_collision * (new_particle.pos.y - old_particle.pos.y)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.y - old_particle.pos.y);
    }
    if new_particle.pos.y + old_particle.radius > screen_height() {
        new_particle.vel.y = -new_particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_height() - old_particle.radius - old_particle.pos.y)
                / (new_particle.pos.y - old_particle.pos.y);

        new_particle.pos.y = old_particle.pos.y
            + fraction_of_trajectory_before_collision * (new_particle.pos.y - old_particle.pos.y)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.y - old_particle.pos.y);
    }
    if new_particle.pos.x - old_particle.radius < 0. {
        new_particle.vel.x = -new_particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision = (0. + old_particle.radius
            - old_particle.pos.x)
            / (new_particle.pos.x - old_particle.pos.x);

        new_particle.pos.x = old_particle.pos.x
            + fraction_of_trajectory_before_collision * (new_particle.pos.x - old_particle.pos.x)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.x - old_particle.pos.x);
    }
    if new_particle.pos.x + old_particle.radius > screen_width() {
        new_particle.vel.x = -new_particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_width() - old_particle.radius - old_particle.pos.x)
                / (new_particle.pos.x - old_particle.pos.x);
        new_particle.pos.x = old_particle.pos.x
            + fraction_of_trajectory_before_collision * (new_particle.pos.x - old_particle.pos.x)
            - (1. - fraction_of_trajectory_before_collision)
                * (new_particle.pos.x - old_particle.pos.x);
    }
    return new_particle;
}

// Return "true" if two bounding boxes intersect, return "false" otherwise.
pub fn intersect(a: AABB, b: AABB) -> bool {
    if a.lower_bound.x > b.upper_bound.x {
        return false;
    } else if a.upper_bound.x < b.lower_bound.x {
        return false;
    } else if a.lower_bound.y > b.upper_bound.y {
        return false;
    } else if a.upper_bound.y < b.lower_bound.y {
        return false;
    } else {
        return true;
    }
}

pub fn handle_particle_collision(a: &Particle, b: &Particle) -> (Vec2, Vec2) //,(Vec2, Vec2))
{
    // let dist = a.pos.distance(b.pos);
    // let min_dist = a.radius + b.radius;
    // if dist < min_dist {
    //     let a_pos_before_collision = a.pos - a.vel;
    //     let b_pos_before_collision = b.pos - b.vel;

    //     let a_negative_travel_vec = a.pos - a_pos_before_collision;
    //     let b_negative_travel_vec = b.pos - b_pos_before_collision;
    // }

    let new_a_vel = a.vel
        - (2. * b.mass / (a.mass + b.mass))
            * (((a.vel - b.vel).dot(a.pos - b.pos)) / (a.pos - b.pos).length_squared())
            * (a.pos - b.pos);

    // let new_a_pos = a.pos + a.vel;

    let new_b_vel = b.vel
        - (2. * a.mass / (a.mass + b.mass))
            * (((b.vel - a.vel).dot(b.pos - a.pos)) / (b.pos - a.pos).length_squared())
            * (b.pos - a.pos);

    // let new_b_pos = b.pos + b.vel;

    return //(
        (new_a_vel, new_b_vel); //, (new_a_pos, new_b_pos));
}
