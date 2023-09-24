use macroquad::prelude::*;
use std::string::ToString;

const RADIUS: f32 = 10.;

const COEF_OF_RESTITUTION: f32 = 0.5;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    // rotation: f32,
    // rotation_speed: f32,
    // charge: f32,
    mass: f32,
}

fn update(dt: f32, particles: &mut Vec<Particle>) {
    for particle in particles.iter_mut() {
        let old_pos = particle.pos;

        particle.vel.x += particle.acc.x * dt;
        particle.vel.y += particle.acc.y * dt;
        particle.pos.x += particle.vel.x * dt;
        particle.pos.y += particle.vel.y * dt;
        particle.acc = particle.acc;

        handle_wall_collisions(particle, old_pos);
    }

    for other_particle in particles.iter() {
        let a = particle;
        let b = other_particle;
        {
            let normal = a.pos - b.pos;
            let unit_normal = normal.normalize();
            let unit_tangent = Vec2::new(unit_normal.y, unit_normal.x);
            let a_normal = a.vel.dot(unit_normal);
            let a_tangent = a.vel.dot(unit_tangent);
            let b_normal = b.vel.dot(unit_normal);
            let b_tangent = b.vel.dot(unit_tangent);

            let a_normal_new =
                (a_normal * (a.mass - b.mass) + 2. * b.mass * b_normal) / (a.mass + b.mass);
            let b_normal_new =
                (b_normal * (b.mass - a.mass) + 2. * a.mass * a_normal) / (a.mass + b.mass);

            let a_normal_new = a_normal_new * unit_normal;
            let a_tangent_new = a_tangent * unit_tangent;
            let b_normal_new = b_normal_new * unit_normal;
            let b_tangent_new = b_tangent * unit_tangent;

            particle.vel.x = a_normal_new.x + b_normal_new.x;
            particle.vel.y = a_normal_new.y + b_normal_new.y;
        }
    }
}

fn handle_wall_collisions(particle: &Particle, old_pos: Vec2) {
    if particle.pos.y - RADIUS < 0. {
        particle.vel.y = -particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (0. + RADIUS - old_pos.y) / (particle.pos.y - old_pos.y);

        particle.pos.y = old_pos.y
            + fraction_of_trajectory_before_collision * (particle.pos.y - old_pos.y)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.y - old_pos.y);
    }
    if particle.pos.y + RADIUS > screen_height() {
        particle.vel.y = -particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_height() - RADIUS - old_pos.y) / (particle.pos.y - old_pos.y);

        particle.pos.y = old_pos.y
            + fraction_of_trajectory_before_collision * (particle.pos.y - old_pos.y)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.y - old_pos.y);
    }
    if particle.pos.x - RADIUS < 0. {
        particle.vel.x = -particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (0. + RADIUS - old_pos.x) / (particle.pos.x - old_pos.x);

        particle.pos.x = old_pos.x
            + fraction_of_trajectory_before_collision * (particle.pos.x - old_pos.x)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.x - old_pos.x);
    }
    if particle.pos.x + RADIUS > screen_width() {
        particle.vel.x = -particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_width() - RADIUS - old_pos.x) / (particle.pos.x - old_pos.x);
        particle.pos.x = old_pos.x
            + fraction_of_trajectory_before_collision * (particle.pos.x - old_pos.x)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.x - old_pos.x);
        // draw_text(
        //     &fraction_of_trajectory_before_collision.to_string(),
        //     particle.pos.x,
        //     particle.pos.y,
        //     50.,
        //     ORANGE,
        // );
    }
}

#[macroquad::main("Particle Simulator")]
async fn main() {
    let mut particles = Vec::new();

    particles.push(Particle {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        vel: Vec2::new(50., 50.),
        acc: Vec2::new(0., 9.81),
        mass: 5.,
    });
    let frame_rate: f32 = 100.;
    let dt: f32 = 1. / frame_rate;

    loop {
        if is_mouse_button_down(MouseButton::Left) == true {
            particles.push(Particle {
                pos: Vec2::new(mouse_position().0, mouse_position().1),
                vel: Vec2::new(500., 5000.),
                acc: Vec2::new(0., 9.81),
                mass: 5.,
            });
        }

        let frame_time = get_frame_time();

        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < dt {
            let time_to_sleep = (dt - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        let fps = 1. / frame_time;

        clear_background(LIGHTGRAY);

        update(dt, &mut particles);

        for particle in particles.iter() {
            draw_circle(particle.pos.x, particle.pos.y, RADIUS, WHITE);

            draw_line(0., 0., particle.pos.x, particle.pos.y, 5., BLACK);

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
        }

        draw_text(&fps.to_string(), 50., 50., 100., BLUE);
        next_frame().await
    }
}
