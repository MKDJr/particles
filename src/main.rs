use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::math::Vec2;
use macroquad::prelude::*;
use std::string::ToString;
mod bounding_box_and_utils;
use bounding_box_and_utils::create_aabb;

use crate::grid::Grid;
use crate::utils::draw_bb;
mod collision_system;
mod grid;
mod update;
mod utils;

const RADIUS: f32 = 25.;
const COEF_OF_RESTITUTION: f32 = 0.95;

#[derive(Debug, Copy, Clone, PartialEq)]
struct AABB {
    lower_bound: Vec2,
    upper_bound: Vec2,
    // area: area(&self)
}

type Particles = Vec<Particle>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    // rotation: f32,
    // rotation_speed: f32,
    // charge: f32,
    mass: f32,
    radius: f32,
    bounding_box: AABB,
    tile: Option<usize>,
}

impl Particle {
    fn new(pos: Vec2, vel: Vec2, acc: Vec2, mass: f32) -> Self {
        let radius = RADIUS;
        let bounding_box = create_aabb(pos.x, pos.y, radius);
        Self {
            pos,
            vel,
            acc,
            mass,
            radius,
            bounding_box,
            tile: None,
        }
    }
}

#[macroquad::main("Particle Simulator")]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let frame_rate: f32 = 100.;
    let dt: f32 = 1. / frame_rate;

    let mut particles: Particles = Particles::new();
    let dur: f32 = 1.;
    let mut curr: f32 = 0.;
    loop {
        if dur < curr {
            if is_mouse_button_down(MouseButton::Left) == true {
                particles.push(Particle::new(
                    Vec2::new(mouse_position().0, mouse_position().1),
                    Vec2::new(
                        1000. * thread_rng().gen_range(-1.0..1.0),
                        1000. * thread_rng().gen_range(-1.0..1.0),
                    ),
                    Vec2::new(0., 9.81),
                    5.,
                ));
                curr = 0.;
            } else {
                // particles.push(Particle::new(
                //     Vec2::new(100f32, 100f32),
                //     Vec2::new(
                //         1000. * thread_rng().gen_range(-1.0..1.0),
                //         1000. * thread_rng().gen_range(-1.0..1.0),
                //     ),
                //     Vec2::new(0., 9.81),
                //     5.,
                // ))
            }
        }

        let frame_time: f32 = get_frame_time();
        let fps: f32 = 1. / frame_time;

        curr += frame_time;

        // println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < dt {
            let time_to_sleep = (dt - frame_time) * 1000.;
            // println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        clear_background(LIGHTGRAY);

        let frozen_particles: Particles = particles.clone();
        particles = update::update_world(&dt, frozen_particles);

        let mut grid = Grid::new(screen_width(), screen_height(), 3, 3);
        let mut i = 0;
        while let Some(tile) = grid.tiles.pop() {
            draw_text(
                &i.to_string(),
                tile.bounding_box.lower_bound.x + 100.,
                tile.bounding_box.lower_bound.y + 100.,
                50.,
                RED,
            );
            draw_bb(tile.bounding_box, BLUE);
            i += 1;
        }
        for particle in particles.iter() {
            draw_circle(particle.pos.x, particle.pos.y, particle.radius, WHITE);

            // pos line
            // draw_line(0., 0., particle.pos.x, particle.pos.y, 5., BLACK);
            // vel line

            draw_bb(particle.bounding_box, BLUE);

            draw_line(
                particle.pos.x,
                particle.pos.y,
                particle.pos.x + particle.vel.x,
                particle.pos.y + particle.vel.y,
                5.,
                RED,
            );
            // acc line
            draw_line(
                particle.pos.x,
                particle.pos.y,
                particle.pos.x + particle.acc.x,
                particle.pos.y + particle.acc.y,
                5.,
                BLUE,
            );
            // pos
            // draw_text(
            //     &particle.pos.x.to_string(),
            //     particle.pos.x,
            //     particle.pos.y,
            //     50.,
            //     ORANGE,
            // );
        }

        draw_text(
            &particles.len().to_string(),
            screen_width() - 150.,
            screen_height() - 50.,
            100.,
            BLUE,
        );

        draw_text(&fps.to_string(), 50., 50., 100., BLUE);
        next_frame().await
    }
}
