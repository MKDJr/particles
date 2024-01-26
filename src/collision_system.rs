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
pub fn bbox_intersect(a: &AABB, b: &AABB) -> bool {
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

// Return "true" if two bounding boxes intersect, return "false" otherwise.
pub fn point_intersect(a: &Particle, b: &AABB) -> bool {
    if
    // particle is farther right than left edge of box
    (a.pos.x > b.lower_bound.x) &&
        // particle is farther left than right edge of box
        (a.pos.x < b.upper_bound.x) &&
        // particle is lower than upper edge of box
        (a.pos.y > b.lower_bound.y) &&
        // particle is higher than the lower edge of the box
        (a.pos.y < b.upper_bound.y)
    {
        return true;
    } else {
        return false;
    }
}

pub fn handle_particle_collision(a: &Particle, b: &Particle) -> (Particle, Particle) //,(Vec2, Vec2))
{
    let d_intersect = a.pos.distance(b.pos);
    let d_min = a.radius + b.radius;

    let t = (d_min - (a.pos - b.pos)) / (a.vel - b.vel);
    dbg!(t);

    let (mut a_collision, mut b_collision) = (a.clone(), b.clone());
    (a_collision.pos, b_collision.pos) = (
        a_collision.pos - a_collision.vel * t,
        b_collision.pos - b_collision.vel * t,
    );

    let (new_a_vel, new_b_vel) = calculate_post_collision_velocity(a, b);

    (a_collision.vel, b_collision.vel) = (new_a_vel, new_b_vel);
    let (mut a_post_collision, mut b_post_collision) = (a_collision.clone(), b_collision.clone());
    (a_post_collision.pos, b_post_collision.pos) = (
        a_collision.pos + a_collision.vel * (1. - t),
        b_collision.pos + b_collision.vel * (1. - t),
    );

    println!("!!!!!!!!!!!!!! COLLISION !!!!!!!!!!!!!!!");
    dbg!(a_post_collision, b_post_collision);

    return //(
        (a_post_collision, b_post_collision); //, (new_a_pos, new_b_pos));
}

/// I am certain the velocity calculation here is wrong.

fn calculate_post_collision_velocity(a: &Particle, b: &Particle) -> (Vec2, Vec2) {
    let new_b_vel = (a.vel * a.mass + b.vel * b.mass) / b.mass;
    let new_a_vel = (b.vel * b.mass + a.vel * a.mass) / a.mass;
    return (new_a_vel, new_b_vel);
    todo!();
}

// r_b + r_a = d_min

// || a_collide.pos - b_collide.pos || = d_collision;

// a_collide.pos = a_intersect.pos - a_intersect.vel * factor1

// b_collide.pos = b_intersect.pos - b_intersect.vel * factor2

// || (a_intersect.pos - a_intersect.vel * factor1) - (b_intersect.pos - b_intersect.vel * factor2) || = r_b + r_a

// (a_intersect.pos - a_intersect.vel * factor1)^2
// - 2*(a_intersect.pos - a_intersect.vel * factor1)*(b_intersect.pos - b_intersect.vel * factor2)
// + (b_intersect.pos - b_intersect.vel * factor2)^2
// = (r_b + r_a)^2 // a number

// (aip - aiv*f1)^2 - 2*(aip - aiv*f1)*(bip - biv*f2) + (bip - biv*f2)^2 = (r_b + r_a)^2

// (aip^2 - 2*aiv*f1 + aiv^2 * f1^2 ) - 2*(aip*bip - aip*biv*f2 - bip*aiv*f1 - biv*aiv*f2*f1) + (bip^2 - 2*biv*f2 +biv^2 * f2^2) = (r_b + r_a)^2

// if f1 = f2
// aip^2 - 2*aiv*f + aiv^2 * f^2 - 2*aip*bip - 2*aip*biv*f - 2*bip*aiv*f - 2*biv*aiv*f^2 + bip^2 - 2*biv*f +biv^2 * f^2 = (r_b + r_a)^2
// aip^2 - 2*aiv*f + aiv^2 * f^2 - 2*aip*bip - 2*aip*biv*f - 2*bip*aiv*f - 2*biv*aiv*f^2 + bip^2 - 2*biv*f +biv^2 * f^2 = (r_b + r_a)^2

// n = number

// n - n*f + n + f^2 - n - n*f - n*f - n*f^2 + n - n*f + n + f^2 = n
// n*f + f^2 - n*f - n*f - n*f^2 +  n*f + f^2 = n
// nf^2 + n*f = n
// f^2 + f = n
