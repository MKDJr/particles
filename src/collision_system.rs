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
    let t = calculate_t(a, b);

    let (mut a_collision, mut b_collision) = (a.clone(), b.clone());
    (a_collision.pos, b_collision.pos) = (a.pos + a.vel * t, b.pos + b.vel * t);

    let (new_a_vel, new_b_vel) =
        calculate_post_collision_velocity(a, b, b_collision.pos - a_collision.pos);

    (a_collision.vel, b_collision.vel) = (new_a_vel, new_b_vel);
    let (mut a_post_collision, mut b_post_collision) = (a_collision.clone(), b_collision.clone());
    (a_post_collision.pos, b_post_collision.pos) = (
        a_collision.pos + a_collision.vel * (1. - t),
        b_collision.pos + b_collision.vel * (1. - t),
    );

    println!("!!!!!!!!!!!!!! COLLISION !!!!!!!!!!!!!!!");
    // dbg!(a_post_collision, b_post_collision);

    return //(
        (a_post_collision, b_post_collision); //, (new_a_pos, new_b_pos));
}

// fraction of time before the particles collide
fn calculate_t(a: &Particle, b: &Particle) -> f32 {
    // From chapter 8 of "Mathematics and Physics for Programmers" by Danny Kodicek

    let w = a.pos - b.pos;
    let r = a.radius + b.radius;
    let ww = w.dot(w);
    // if ww < r * r {
    //     panic!("embedded");
    // }
    let v = a.vel - b.vel;
    let aaa = v.dot(v);
    let bbb = w.dot(v);
    let c = ww - r * r;
    let root = bbb * bbb - aaa * c;
    if root < 0f32 {
        dbg!(root);
        panic!("none");
    }
    let t = (-bbb - f32::sqrt(root)) / aaa;
    if t > 1f32 {
        dbg!(t);
        panic!("none");
    }

    // let a_con = (a.vel - b.vel).length_squared();
    // let b_con = 2f32 * ((a.pos - b.pos).dot(a.vel - b.vel));
    // let c_con = (a.pos - b.pos).length_squared() - d_min;
    // let t = -b_con - f32::sqrt(b_con.powi(2) - 4. * a_con * c_con) / (2. * a_con);
    // dbg!(t);
    return t;
}

/// I am certain the velocity calculation here is wrong.

fn calculate_post_collision_velocity(
    a: &Particle,
    b: &Particle,
    collision_vector: Vec2,
) -> (Vec2, Vec2) {
    let r = a.mass / b.mass;
    let u = a.vel - b.vel;

    let un = u.project_onto(collision_vector);
    let ut = u - un;
    let vn = un * (r - 1.) / (r + 1.);
    let wn = un * 2. * r / (r + 1.);
    let new_a_vel = ut + vn + b.vel;
    let new_b_vel = wn + b.vel;

    return (new_a_vel, new_b_vel);
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
