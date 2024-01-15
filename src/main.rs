use macroquad::math::Vec2;
use macroquad::prelude::*;
use std::string::ToString;

mod bounding_box_and_utils;
use bounding_box_and_utils::create_aabb;
mod collision_system;
mod grid;
mod update;

const RADIUS: f32 = 10.;
const COEF_OF_RESTITUTION: f32 = 0.5;

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
        let radius = 5.0;
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

    let mut particles = Particles::new();
    let dur = 0.2;
    let mut curr = 0.;
    loop {
        if dur < curr {
            curr = 0.;
        if is_mouse_button_down(MouseButton::Left) == true {
                particles.push(Particle::new(
                Vec2::new(mouse_position().0, mouse_position().1),
                Vec2::new(
                    1000. * mouse_delta_position().x,
                    1000. * mouse_delta_position().y,
                ),
                Vec2::new(0., 9.81),
                5.,
            ));
            }
        }

        let frame_time = get_frame_time();
        let fps = 1. / frame_time;

        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < dt {
            let time_to_sleep = (dt - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        clear_background(LIGHTGRAY);

        let frozen_particles = particles.clone();
        particles = update::update_world(&dt, frozen_particles);

        // update_tree_aabbs(&mut tree, &particles);
        // draw_text(
        //     &tree.len().to_string(),
        //     screen_width() - 150.,
        //     50.,
        //     100.,
        //     BLUE,
        // );

        // for node in tree.iter() {
        //     draw_rectangle_lines(
        //         node.aabb.lower_bound.x,
        //         node.aabb.lower_bound.y,
        //         node.aabb.upper_bound.x - node.aabb.lower_bound.x,
        //         node.aabb.upper_bound.y - node.aabb.lower_bound.y,
        //         5.,
        //         BLACK,
        //     )
        // }

        for (idx, particle) in particles.list.iter() {
            draw_circle(particle.pos.x, particle.pos.y, RADIUS, WHITE);

            // pos line
            // draw_line(0., 0., particle.pos.x, particle.pos.y, 5., BLACK);

            // vel line
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
            &particles.list.len().to_string(),
            screen_width() - 150.,
            screen_height() - 50.,
            100.,
            BLUE,
        );

        draw_text(&fps.to_string(), 50., 50., 100., BLUE);
        next_frame().await
    }
}
