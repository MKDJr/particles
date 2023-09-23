use macroquad::prelude::*;

const RADIUS: f32 = 10.;

const COEF_OF_RESTITUTION: f32 = 0.5;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2, // rotation: f32,
               // rotation_speed: f32,
               // charge: f32,
               // mass: f32,
}

fn update(dt: f32, particle: &mut Particle) {
    let old_pos = particle.pos;

    particle.vel.x += particle.acc.x * dt;
    particle.vel.y += particle.acc.y * dt;
    particle.pos.x += particle.vel.x * dt;
    particle.pos.y += particle.vel.y * dt;
    particle.acc = particle.acc;

    handle_collisions(particle, old_pos);
}

fn handle_collisions(particle: &mut Particle, old_pos: Vec2) {
    if particle.pos.y < 0. {
        particle.vel.y = -particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (0. + RADIUS - old_pos.y) / (particle.pos.y - old_pos.y);

        particle.pos.y += fraction_of_trajectory_before_collision * (particle.pos.y - old_pos.y)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.y - old_pos.y);
    }
    if particle.pos.y > screen_height() {
        particle.vel.y = -particle.vel.y * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_height() - RADIUS - particle.pos.y) / (particle.pos.y - old_pos.y);

        particle.pos.y += fraction_of_trajectory_before_collision * (particle.pos.y - old_pos.y)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.y - old_pos.y);
    }

    if particle.pos.x < 0. {
        particle.vel.x = -particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (0. + RADIUS - particle.pos.x) / (particle.pos.x - old_pos.x);

        particle.pos.x += fraction_of_trajectory_before_collision * (particle.pos.x - old_pos.x)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.x - old_pos.x);
    }
    if particle.pos.x > screen_width() {
        particle.vel.x = -particle.vel.x * COEF_OF_RESTITUTION;

        let fraction_of_trajectory_before_collision =
            (screen_width() - RADIUS - particle.pos.x) / (particle.pos.x - old_pos.x);

        particle.pos.x += fraction_of_trajectory_before_collision * (particle.pos.x - old_pos.x)
            - (1. - fraction_of_trajectory_before_collision) * (particle.pos.x - old_pos.x);
    }
}

#[macroquad::main("Particle Simulator")]
async fn main() {
    let mut particle = Particle {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        vel: Vec2::new(50., 50.),
        acc: Vec2::new(0., 9.81 * 100.),
    };
    let frame_rate: f32 = 60.;
    let dt: f32 = 1. / frame_rate;

    loop {
        let frame_time = get_frame_time();
        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < dt {
            let time_to_sleep = (dt - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        clear_background(LIGHTGRAY);

        update(dt, &mut particle);

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
        next_frame().await
    }
}
