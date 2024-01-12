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
