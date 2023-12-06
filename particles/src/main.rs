use macroquad::prelude::*;
use std::collections::HashMap;
use std::string::ToString;
const RADIUS: f32 = 10.;

mod update_system;

const COEF_OF_RESTITUTION: f32 = 0.5;

/* -------------------------------------------------------------------------- */
/*                                   Structs                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Copy, Clone, PartialEq)]

struct AABB {
    lower_bound: Vec2,
    upper_bound: Vec2,
    // area: area(&self)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    // rotation: f32,
    // rotation_speed: f32,
    // charge: f32,
    mass: f32,
}

/* -------------------------------------------------------------------------- */
/*                                 Grid System                                */
/* -------------------------------------------------------------------------- */

mod grid_system {
    use crate::{Particle, AABB};
    use macroquad::math::Vec2;
    use std::collections::HashMap;

    // hashmap of grid index and bbox
    fn create_grid(width: f32, height: f32, n_x: i32, n_y: i32) -> HashMap<usize, AABB> {
        let mut grid: HashMap<usize, AABB> = HashMap::new();
        for num_x in 0..n_x {
            for num_y in 0..n_y {
                grid.insert(
                    (num_x * n_x + num_y).try_into().unwrap(),
                    AABB {
                        lower_bound: Vec2 {
                            x: (width * num_x as f32 / n_x as f32),
                            y: (height * num_y as f32 / n_y as f32),
                        },
                        upper_bound: Vec2 {
                            x: (width * (num_x as f32 + 1.) / n_x as f32),
                            y: (height * (num_y as f32 + 1.) / n_y as f32),
                        },
                    },
                );
            }
        }
        return grid;
    }

    // hashmap of grid index and particle
    // search grid and create vec for each particle with pointers to the grid square they're in
    fn which_square_is_particle_in(
        particles: &Vec<Particle>,
        grid: HashMap<usize, AABB>,
    ) -> HashMap<usize, Particle> {
        let mut hashmap_of_particle_and_grid_index_its_in: HashMap<usize, Particle> =
            HashMap::new();
        for particle in particles.iter() {
            // static
            for (index, square) in grid.iter() {
                // linear
                if intersect(create_aabb_particle(particle), *square) {}
                // could store aabb with particle
                hashmap_of_particle_and_grid_index_its_in.insert(index.clone(), particle.clone());
            }
        }
        return hashmap_of_particle_and_grid_index_its_in;
    }

    // Hey! I have a grid for the windows with indeces and bounding boxes.
    // If you give me a list of particles, I'll tell you which ones are in the same grid square so you can check them for collissions.
    // I just need to borrow the particles, I won't be changing them.
}

mod collision_system {
    // take in particle location HashMap, grid hashmap, and particles vec
    // return
    fn check_for_collisions(
        hashmap_of_particle_and_grid_index_its_in: HashMap<usize, Particle>,
        grid: HashMap<usize, Particle>,
        particles: &Vec<Particle>,
    ) -> HashMap<usize, usize> {
        let mut collisions = HashMap::new();

        for square_index in hashmap_of_particle_and_grid_index_its_in.keys() {}

        // for index of grid square, and area...
        for (index, _square) in grid.iter() {
            // create stack of particles in this grid square
            // let mut set_of_indeces: Vec<usize> = hashmap_of_particle_and_grid_index_its_in.clone();
            // set_of_indeces.retain(|&x| x == index);
            // let mut particle_stack = get_vector_subset(particles, &set_of_indeces);

            // check which particles intersect
            while !particle_stack.is_empty() {
                let particle_a = particle_stack.pop().unwrap();
                let mut particle_stack_working = particle_stack.clone();
                for particle_b in particle_stack_working.iter_mut() {
                    if intersect(
                        create_aabb_particle_owned(particle_a),
                        create_aabb_particle(particle_b),
                    ) {
                        collisions.insert(particle_a, particle_b)
                    }
                }
            }
        }
        return collisions;
    }

    fn handle_wall_collisions(new_particle: &mut Particle, old_particle: &Particle) {
        if new_particle.pos.y - RADIUS < 0. {
            new_particle.vel.y = -new_particle.vel.y * COEF_OF_RESTITUTION;

            let fraction_of_trajectory_before_collision =
                (0. + RADIUS - old_particle.pos.y) / (new_particle.pos.y - old_particle.pos.y);

            new_particle.pos.y = old_particle.pos.y
                + fraction_of_trajectory_before_collision
                    * (new_particle.pos.y - old_particle.pos.y)
                - (1. - fraction_of_trajectory_before_collision)
                    * (new_particle.pos.y - old_particle.pos.y);
        }
        if new_particle.pos.y + RADIUS > screen_height() {
            new_particle.vel.y = -new_particle.vel.y * COEF_OF_RESTITUTION;

            let fraction_of_trajectory_before_collision =
                (screen_height() - RADIUS - old_particle.pos.y)
                    / (new_particle.pos.y - old_particle.pos.y);

            new_particle.pos.y = old_particle.pos.y
                + fraction_of_trajectory_before_collision
                    * (new_particle.pos.y - old_particle.pos.y)
                - (1. - fraction_of_trajectory_before_collision)
                    * (new_particle.pos.y - old_particle.pos.y);
        }
        if new_particle.pos.x - RADIUS < 0. {
            new_particle.vel.x = -new_particle.vel.x * COEF_OF_RESTITUTION;

            let fraction_of_trajectory_before_collision =
                (0. + RADIUS - old_particle.pos.x) / (new_particle.pos.x - old_particle.pos.x);

            new_particle.pos.x = old_particle.pos.x
                + fraction_of_trajectory_before_collision
                    * (new_particle.pos.x - old_particle.pos.x)
                - (1. - fraction_of_trajectory_before_collision)
                    * (new_particle.pos.x - old_particle.pos.x);
        }
        if new_particle.pos.x + RADIUS > screen_width() {
            new_particle.vel.x = -new_particle.vel.x * COEF_OF_RESTITUTION;

            let fraction_of_trajectory_before_collision =
                (screen_width() - RADIUS - old_particle.pos.x)
                    / (new_particle.pos.x - old_particle.pos.x);
            new_particle.pos.x = old_particle.pos.x
                + fraction_of_trajectory_before_collision
                    * (new_particle.pos.x - old_particle.pos.x)
                - (1. - fraction_of_trajectory_before_collision)
                    * (new_particle.pos.x - old_particle.pos.x);
        }
    }
}

mod bounding_box_utils {
    use crate::{Particle, AABB, RADIUS};
    use macroquad::math::Vec2;

    fn create_aabb_particle(particle: &Particle) -> AABB {
        let bbox = AABB {
            lower_bound: Vec2::new(particle.pos.x - RADIUS, particle.pos.y - RADIUS),
            upper_bound: Vec2::new(particle.pos.x + RADIUS, particle.pos.y + RADIUS),
        };
        return bbox;
    }

    fn create_aabb_particle_owned(particle: Particle) -> AABB {
        let bbox = AABB {
            lower_bound: Vec2::new(particle.pos.x - RADIUS, particle.pos.y - RADIUS),
            upper_bound: Vec2::new(particle.pos.x + RADIUS, particle.pos.y + RADIUS),
        };
        return bbox;
    }

    fn create_aabb_particle_group(particle_group: &Vec<Particle>) -> AABB {
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

    // Return "true" if two bounding boxes intersect, return "false" otherwise.
    fn intersect(a: AABB, b: AABB) -> bool {
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
}

fn get_vector_subset(vector: &Vec<Particle>, indeces_to_get: &Vec<usize>) -> Vec<Particle> {
    let mut new_vector = Vec::new();
    for index in indeces_to_get.iter() {
        new_vector.push(vector[*index])
    }
    return new_vector;
}

fn update(dt: &f32, old_particles: &Vec<Particle>) -> Vec<Particle> {
    // clone list to get new particle list and return that

    let mut new_particles = old_particles.clone();

    // calculate new positions
    for new_particle in new_particles.iter_mut() {
        new_particle.vel.x += new_particle.acc.x * dt;
        new_particle.vel.y += new_particle.acc.y * dt;
        new_particle.pos.x += new_particle.vel.x * dt;
        new_particle.pos.y += new_particle.vel.y * dt;
        new_particle.acc = new_particle.acc;
    }

    // check if any hit the walls and adjust
    for (new_particle, old_particle) in new_particles.iter_mut().zip(old_particles.iter()) {
        handle_wall_collisions(new_particle, old_particle)
    }

    let grid: Vec<AABB> = create_grid(screen_width(), screen_height(), 9, 9);
    let presence: Vec<usize> = which_square_is_particle_in(&new_particles, grid);

    return new_particles;
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
    let mut tree = vec![Node {
        aabb: create_aabb_particle_group(&particles),
        kind: NodeKind::Root {
            indeces_of_particle_group: (0..particles.len()).collect(),
            left_child_index: None,
            right_child_index: None,
        },
    }];
    // aabb: create_aabb_particle_group(&particles),
    // particle_index: particles
    //     .iter()
    //     .position(|&p| p == *particles.last().unwrap())
    //     .unwrap(),;

    loop {
        if is_mouse_button_down(MouseButton::Left) == true {
            particles.push(Particle {
                pos: Vec2::new(mouse_position().0, mouse_position().1),
                vel: Vec2::new(500., 5000.),
                acc: Vec2::new(0., 9.81),
                mass: 5.,
            });
            // update_tree_when_new_particle(&mut tree, &particles);
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
        particles = update(&dt, &particles);
        // update_tree_aabbs(&mut tree, &particles);
        draw_text(
            &tree.len().to_string(),
            screen_width() - 150.,
            50.,
            100.,
            BLUE,
        );

        for node in tree.iter() {
            draw_rectangle_lines(
                node.aabb.lower_bound.x,
                node.aabb.lower_bound.y,
                node.aabb.upper_bound.x - node.aabb.lower_bound.x,
                node.aabb.upper_bound.y - node.aabb.lower_bound.y,
                5.,
                BLACK,
            )
        }

        for particle in particles.iter() {
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
            draw_text(
                &particle.pos.x.to_string(),
                particle.pos.x,
                particle.pos.y,
                50.,
                ORANGE,
            );
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
