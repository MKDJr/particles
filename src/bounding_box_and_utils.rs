use macroquad::math::Vec2;

use crate::{Particle, AABB, RADIUS};

pub fn create_aabb(x: f32, y: f32, radius: f32) -> AABB {
    let bbox = AABB {
        lower_bound: Vec2::new(x - radius, y - radius),
        upper_bound: Vec2::new(x + radius, y + radius),
    };
    return bbox;
}

pub fn create_aabb_particle_group(particle_group: &Vec<Particle>) -> AABB {
    let mut bbox = AABB {
        lower_bound: Vec2::new(0., 0.),
        upper_bound: Vec2::new(0., 0.),
    };
    bbox.upper_bound.x = particle_group
        .iter()
        .max_by(|p1, p2| p1.pos.x.partial_cmp(&p2.pos.x).unwrap())
        .unwrap()
        .pos
        .x
        + RADIUS;
    bbox.lower_bound.x = particle_group
        .iter()
        .min_by(|p1, p2| p1.pos.x.partial_cmp(&p2.pos.x).unwrap())
        .unwrap()
        .pos
        .x
        - RADIUS;
    bbox.upper_bound.y = particle_group
        .iter()
        .max_by(|p1, p2| p1.pos.y.partial_cmp(&p2.pos.y).unwrap())
        .unwrap()
        .pos
        .y
        + RADIUS;
    bbox.lower_bound.y = particle_group
        .iter()
        .min_by(|p1, p2| p1.pos.y.partial_cmp(&p2.pos.y).unwrap())
        .unwrap()
        .pos
        .y
        - RADIUS;

    return bbox;
}

// Calculate the union of two bounding boxes.
fn union(a: AABB, b: AABB) -> AABB {
    let mut c: AABB = AABB {
        lower_bound: Vec2 { x: 0., y: 0. },
        upper_bound: Vec2 { x: 0., y: 0. },
    };
    c.lower_bound.x = f32::min(a.lower_bound.x, b.lower_bound.x);
    c.lower_bound.y = f32::min(a.lower_bound.y, b.lower_bound.y);
    c.upper_bound.x = f32::max(a.upper_bound.x, b.upper_bound.x);
    c.upper_bound.y = f32::max(a.upper_bound.x, b.upper_bound.y);
    return c;
}

// Calculate the area of a bounding box.
fn area(a: AABB) -> f32 {
    return (a.upper_bound.x - a.lower_bound.x) * (a.upper_bound.y - a.lower_bound.y);
}
