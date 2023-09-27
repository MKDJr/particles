use core::num;
use macroquad::prelude::*;
use ordered_float::{self, OrderedFloat};
use std::borrow::BorrowMut;
use std::cmp::max;
use std::cmp::min;
use std::collections::*;
use std::string::ToString;
use std::{cell::RefCell, rc::Rc};
const RADIUS: f32 = 10.;

const COEF_OF_RESTITUTION: f32 = 0.5;

#[derive(Debug, Copy, Clone)]
struct AABB {
    lower_bound: Vec2,
    upper_bound: Vec2,
    // area: area(&self)
}

fn create_aabb(particle_group: Vec<Particle>) -> AABB {
    let mut bbox = AABB {
        lower_bound: Vec2::new(0., 0.),
        upper_bound: Vec2::new(0., 0.),
    };
    bbox.upper_bound.x = particle_group
        .iter()
        .max_by_key(|p| OrderedFloat(p.pos.x))
        .unwrap()
        .pos
        .x
        + RADIUS;
    bbox.lower_bound.x = particle_group
        .iter()
        .min_by_key(|p| OrderedFloat(p.pos.x))
        .unwrap()
        .pos
        .x
        - RADIUS;
    bbox.upper_bound.y = particle_group
        .iter()
        .max_by_key(|p| OrderedFloat(p.pos.y))
        .unwrap()
        .pos
        .x
        + RADIUS;
    bbox.lower_bound.y = particle_group
        .iter()
        .min_by_key(|p| OrderedFloat(p.pos.y))
        .unwrap()
        .pos
        .x
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

//TODO (Change this to use https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/);
fn surface_area(tree: Tree) -> f32 {
    let mut surface_area = 0.;
    for node in tree.iter() {
        surface_area += node.bbox.area
    }
    return surface_area;
}

fn check_if_in_tree(particle: Particle, tree: Tree) {
    match tree.get(particle) {
        Some(node) => return true,
        None => return false,
    }
}

//TODO seperate tree progression to seperate function
fn place_node(tree: Tree, particle: Particle) {
    // let mut sum = 0i32;
    // We'll use a `vec` as a
    // stack LIFO data structure.
    // Start by adding the root node
    // to the stack.
    let mut stack = vec![tree.root];

    while !stack.is_empty() {
        // current points to top most
        // item in the stack
        let current = stack.pop().unwrap();

        // Add particle to current bounding box and recalculate area
        current.borrow_mut().particles.push(particle.clone());
        current.borrow_mut().bbox = create_aabb(current.borrow().particles);

        let mut left_test_area: f32 = -1.;
        // for the two children, check which group the new particle would have to join to minimize total surface area
        if let Some(left) = &current.borrow().left {
            let mut left_test_particles = left.borrow().particles.clone();
            left_test_particles.push(particle.clone());
            let left_test_bbox = create_aabb(left_test_particles);
            left_test_area = area(left_test_bbox);

            let mut right_test_area: f32 = -1.;
            if let Some(right) = &current.borrow().right {
                let mut right_test_particles = right.borrow().particles.clone();
                right_test_particles.push(particle.clone());
                let right_test_bbox = create_aabb(right_test_particles);
                let right_area = area(right_test_bbox);

                if left_test_area > right_test_area {
                    stack.push(right.to_owned())
                } else if right_test_area > left_test_area {
                    stack.push(left.to_owned())
                }
            } else {
                let Some(right) = &current.borrow().right;
                stack.push(right.to_owned());
            }
        } else {
            let particles = vec![particle];
            *current = Node {
                particles: particles,
                bbox: create_aabb(particles),
                left: None,
                right: None,
            };
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    // rotation: f32,
    // rotation_speed: f32,
    // charge: f32,
    mass: f32,
}

#[derive(Debug, Clone)]

struct Node {
    particles: Vec<Particle>, //value
    bbox: AABB,
    // index: i32,
    // is_leaf: bool,
    // parent: Option<Box<Node>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

// impl Node {
//     fn new()
// }
struct Tree {
    root: Rc<RefCell<Node>>,
}

// impl Tree {
//     fn new() -> Self {
//         Tree {root: None}
//     }
// }

// fn search_tree(tree: Tree) {
//     let mut heap = BinaryHeap::new();
//     heap.push(tree.root_index);
//     while heap.is_empty() == false {
//         let index = heap.pop();
//     }

// }

// fn insert_leaf(tree: Tree, particle_index: i32, bbox: AABB ) {
//     let leaf_index = allocate_leaf_node(tree, particle_index, bbox);
//     if tree.node_count == 1 {
//         tree.root_index = leafIndex;
//     }

//     // Stage 1: find the best sibling for the new leaf
//     let best_sibling: i32 = 0;
//     for 0..
//     // Stage 2: create a new parent
//     // Stage 3: walk back up the tree refitting AABBs

// }

fn update(dt: f32, old_particles: &Vec<Particle>) {
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

    let num_particles = particles.len();

    let mut particles_clone = particles.clone();

    fn collision_check(&mut self, particles: &Vec<Particle>) {
        for particle in particles.iter_mut() {
            let a = &mut *self;
            let b = &mut *particle;

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

            self.vel.x = a_normal_new.x + a_tangent_new.x;
            self.vel.y = a_normal_new.y + a_tangent_new.y;
            particle.vel.x = b_normal_new.x + b_tangent_new.x;
            particle.vel.y = b_normal_new.y + b_tangent_new.y;
        }
    }
}

// fn find_collisions(old_particles: &Vec<Particle>) //-> Vec<i32>
// {
//     let mut new_particles = old_particles.clone();

//     let mut root = TreeNode::new(new_particles);
//     let mut tree = BTreeMap::new();
//     tree.insert(1, TreeNode)

//     let sorted_particles = new_particles.sort_by(|d1: &Particle, d2| d1.pos.x.partial_cmp(&d2.pos.x).unwrap());

// /*
// loop {
//     if node.particles.len() == 2 {}
//     else if node.particles.len() > 2 {
//         break it up
//     }
//     if node.left {node = node.left}
//     if node.right {}
// }
// */
// // if bbox_tree_node.particles.len() == 2 {bbox_tree_node.leaf = true}

//     loop {
//         if &axis == "x" {
//             if sorted_particles.len() > 2 {
//                 let median = sorted_particles.len() / 2;
//                 let (a_group, b_group) = sorted_particles.split_at(median);
//                 let a_bbox = create_bbox(a_group);
//                 let b_bbox = create_bbox(b_group);

//             } else if sorted_particles.len() == 2 {
//                 if a_bbox.xmax > b_bbox.xmin {

//                     bbox_tree_parent.left(TreeNode::new(a_group)
//                     bbox_tree_parent.right_child = TreeNode{particles: b_group, bbox: b_bbox, left_child: (), right_child: ()};
//                 }

//             }

//             axis == "y"
//         }
//         else if &axis == "y" {
//             sorted_particles.sort_by(|d1: &Particle, d2| d1.pos.y.partial_cmp(&d2.pos.y).unwrap());
//             let median = sorted_particles.len() / 2;
//             let (a_group, b_group) = sorted_particles.split_at(median);
//             let a_bbox = create_bbox(a_group);
//             let b_bbox = create_bbox(b_group);

//             if a_bbox.ymax > b_bbox.ymin {
//                 collided_particles.push((a_group.iter(), b_group.iter()))
//             }

//              axis == "x"
//         }

//     }

//     bbox_tree_root.left_child = TreeNode{particles: a_group, bbox: a_bbox, left_child: (), right_child: ()};
//     bbox_tree_root.left_child = TreeNode{particles: b_group, bbox: b_bbox, left_child: (), right_child: ()};

//     bbox_tree_root.left_child = TreeNode{particles: a_group, bbox: a_bbox, left_child: (), right_child: ()};
//     bbox_tree_root.left_child = TreeNode{particles: b_group, bbox: b_bbox, left_child: (), right_child: ()};

//     // -------
//     sorted_particles = a_group.to_vec().clone();
//     sorted_particles.sort_by(|d1: &Particle, d2| d1.pos.x.partial_cmp(&d2.pos.x).unwrap());
//     let median = sorted_particles.len() / 2;
//     let (a_group, b_group) = sorted_particles.split_at(median);
//     let a_bbox = create_bbox(a_group);
//     let b_bbox = create_bbox(b_group);

//     if a_bbox.xmax > b_bbox.xmin {
//         collided_particles.push((a_group.iter(), b_group.iter()))
//     }

//     for bbox in bboxes.iter() {

//     // find median
//     // create box around each group
//     // switch axis
//     // repeat

//     // if two of the lowest level boxes intersect
//     // collission_indeces.push()

//     // return collission_indeces;
// }

fn handle_wall_collisions(new_particle: &mut Particle, old_particle: &Particle) {
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
        let fps = 1. / frame_time;

        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < dt {
            let time_to_sleep = (dt - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        clear_background(LIGHTGRAY);

        particles = update(dt, &particles);

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
            screen_width() - 50.,
            screen_height() - 50.,
            100.,
            BLUE,
        );

        draw_text(&fps.to_string(), 50., 50., 100., BLUE);
        next_frame().await
    }
}
