use macroquad::{color::Color, shapes::draw_rectangle_lines};

use crate::{Particle, AABB};

fn get_vector_subset(vector: &Vec<Particle>, indeces_to_get: &Vec<usize>) -> Vec<Particle> {
    let mut new_vector = Vec::new();
    for index in indeces_to_get.iter() {
        new_vector.push(vector[*index])
    }
    return new_vector;
}

pub fn draw_bb(bounding_box: AABB, color: Color) {
    draw_rectangle_lines(
        bounding_box.lower_bound.x,
        bounding_box.lower_bound.y,
        bounding_box.upper_bound.x - bounding_box.lower_bound.x,
        bounding_box.upper_bound.y - bounding_box.lower_bound.y,
        5.,
        color,
    )
}
