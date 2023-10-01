use core::num;
use macroquad::prelude::*;
use ordered_float::{self, OrderedFloat};
use std::borrow::BorrowMut;
use std::cmp::max;
use std::cmp::min;
use std::collections::*;
use std::string::ToString;
use std::thread::current;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};
const RADIUS: f32 = 10.;

const COEF_OF_RESTITUTION: f32 = 0.5;

#[derive(Debug, Copy, Clone, PartialEq)]
struct AABB {
    lower_bound: Vec2,
    upper_bound: Vec2,
    // area: area(&self)
}

fn create_aabb_particle(particle: &Particle) -> AABB {
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

// //TODO (Change this to use https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/);
// fn surface_area(tree: Tree) -> f32 {
//     let mut surface_area = 0.;
//     for node in tree.iter() {
//         surface_area += node.bbox.area
//     }
//     return surface_area;
// }

// fn check_if_in_tree(particle: Particle, tree: Tree) {
//     match tree.get(particle) {
//         Some(node) => return true,
//         None => return false,
//     }
// }

// TODO have to also be able to move these guys around the tree to reorganize
fn update_tree_aabbs(tree: &mut Vec<Node>, particles: &Vec<Particle>) {
    let mut stack: Vec<usize> = (0..tree.len()).collect();
    let mut current_node_kind = NodeKind::Leaf {
        parent_index: 0,
        particle_index: 0,
    };
    while !stack.is_empty() {
        let current_node_index = stack.pop().unwrap();
        {
            current_node_kind = tree[current_node_index].kind.clone()
        }
        match current_node_kind {
            NodeKind::Leaf {
                parent_index: _,
                particle_index,
            } => tree[current_node_index].aabb = create_aabb_particle(&particles[particle_index]),
            NodeKind::InternalNode {
                parent_index: _,
                indeces_of_particle_group,
                left_child_index: _,
                right_child_index: _,
            } => {
                let mut particle_group = Vec::new();
                for index in indeces_of_particle_group.iter() {
                    particle_group.push(particles[*index])
                }
                tree[current_node_index].aabb = create_aabb_particle_group(&particle_group)
            }
            NodeKind::Root {
                indeces_of_particle_group,
                left_child_index: _,
                right_child_index: _,
            } => {
                let mut particle_group = Vec::new();
                for index in indeces_of_particle_group.iter() {
                    particle_group.push(particles[*index])
                }
                tree[current_node_index].aabb = create_aabb_particle_group(&particle_group)
            }
        }
    }
}

fn update_tree_when_new_particle(tree: &mut Vec<Node>, particles: &Vec<Particle>) {
    let now = Instant::now();
    // let mut tree_len = 0usize;

    let root_index = tree
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| area(a.aabb).partial_cmp(&area(b.aabb)).unwrap())
        .unwrap()
        .0;

    // tree.sort_by(|a, b| area(a.aabb).partial_cmp(&area(b.aabb)).unwrap());
    // tree_len = tree.len();

    let mut stack = vec![root_index]; //put root index in there

    let new_particle = particles.last().unwrap();
    let new_particle_bbox = create_aabb_particle(new_particle);
    let new_particle_index = particles.iter().position(|p| p == new_particle);

    while !stack.is_empty() {
        let current_node_index = stack.pop().unwrap();
        let mut current_node_kind: NodeKind = NodeKind::Leaf {
            parent_index: 0,
            particle_index: 0,
        };
        {
            current_node_kind = tree[current_node_index].kind.clone()
        }
        // what type of node it is
        match current_node_kind {
            NodeKind::Leaf {
                ref mut parent_index,
                particle_index,
            } => {
                // create new node and make it's leaves the new leaves

                let new_internal_node = Node {
                    aabb: union(new_particle_bbox, tree[current_node_index].aabb),
                    kind: NodeKind::InternalNode {
                        parent_index: (*parent_index),
                        indeces_of_particle_group: (vec![
                            particle_index,
                            new_particle_index.unwrap(),
                        ]),
                        left_child_index: tree
                            .iter()
                            .position(|n| n == &tree[current_node_index])
                            .unwrap(),
                        right_child_index: tree.len() + 1,
                    },
                };
                tree.push(new_internal_node);

                // make this leaf's parent the new node
                *parent_index = tree.iter().position(|n| n == tree.last().unwrap()).unwrap();

                // create leaf from new particle

                let new_leaf = Node {
                    aabb: new_particle_bbox,
                    kind: NodeKind::Leaf {
                        parent_index: tree.iter().position(|n| n == tree.last().unwrap()).unwrap(),
                        particle_index: new_particle_index.unwrap(),
                    },
                };
                // new_leaf_copy = new_leaf.clone();
                tree.push(new_leaf);

                // let new_leaf_index = tree.iter().position(|n| n == &new_leaf_copy).unwrap();
                // let new_node_index = tree.iter().position(|n| n == &new_node_copy).unwrap();
                // {
                //     let new_leaf = &mut tree[new_leaf_index];

                //     if let NodeKind::Leaf {
                //         ref mut parent_index,
                //         particle_index: _,
                //     } = new_leaf.kind
                //     {
                //         *parent_index = new_node_index;
                //     }
                // }
                // {
                //     let new_node = &mut tree[new_node_index];
                //     if let NodeKind::InternalNode {
                //         parent_index: _,
                //         indeces_of_particle_group: _,
                //         left_child_index: _,
                //         ref mut right_child_index,
                //     } = new_node.kind
                //     {
                //         *right_child_index = new_leaf_index;
                //     }
                // }
            }
            NodeKind::InternalNode {
                parent_index: _,
                mut indeces_of_particle_group,
                left_child_index,
                right_child_index,
            } => {
                indeces_of_particle_group.push(new_particle_index.unwrap());
                //calculate areas
                let left_theoretical_area =
                    area(union(new_particle_bbox, tree[left_child_index].aabb));
                println!("left_theoretical_area: {}", left_theoretical_area);
                let right_theoretical_area =
                    area(union(new_particle_bbox, tree[right_child_index].aabb));
                println!("right_theoretical_area: {}", right_theoretical_area);

                // compare areas
                if left_theoretical_area > right_theoretical_area {
                    stack.push(right_child_index);
                    println!("move to right child");
                } else {
                    stack.push(left_child_index);
                    println!("move to left child");
                }
            }
            NodeKind::Root {
                mut indeces_of_particle_group,
                ref mut left_child_index,
                ref mut right_child_index,
            } => {
                // if root, add new particle to indeces stored by root
                indeces_of_particle_group.push(new_particle_index.unwrap());

                //
                if let (Some(left_child), Some(right_child)) =
                    (*left_child_index, *right_child_index)
                {
                    let left_theoretical_area =
                        area(union(new_particle_bbox, tree[left_child].aabb));
                    let right_theoretical_area =
                        area(union(new_particle_bbox, tree[right_child].aabb));

                    // compare areas
                    if left_theoretical_area > right_theoretical_area {
                        stack.push(right_child);
                    } else {
                        stack.push(left_child)
                    }
                } else if let Some(left_child) = left_child_index {
                    *right_child_index = new_particle_index;
                    let new_leaf = Node {
                        aabb: new_particle_bbox,
                        kind: NodeKind::Leaf {
                            parent_index: current_node_index,
                            particle_index: new_particle_index.unwrap(),
                        },
                    };
                    tree.push(new_leaf)
                } else {
                    *left_child_index = new_particle_index;
                    let new_leaf = Node {
                        aabb: new_particle_bbox,
                        kind: NodeKind::Leaf {
                            parent_index: current_node_index,
                            particle_index: new_particle_index.unwrap(),
                        },
                    };
                    tree.push(new_leaf)
                }
            }
        }
    }
    println!(
        "{} ns to add new particle to tree",
        now.elapsed().as_nanos()
    )
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

#[derive(Debug, Clone, PartialEq)]

struct Node {
    aabb: AABB,
    kind: NodeKind,
}

#[derive(PartialEq, Debug, Clone)]
enum NodeKind {
    Leaf {
        parent_index: usize,
        // aabb: AABB,
        particle_index: usize,
    },
    InternalNode {
        parent_index: usize,
        indeces_of_particle_group: Vec<usize>,
        // aabb: AABB,
        left_child_index: usize,
        right_child_index: usize,
    },
    Root {
        // aabb: AABB,
        indeces_of_particle_group: Vec<usize>,
        left_child_index: Option<usize>,
        right_child_index: Option<usize>,
    },
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

    return new_particles;
    // let num_particles = particles.len();

    // let mut particles_clone = particles.clone();

    // fn collision_check(&mut self, particles: &Vec<Particle>) {
    //     for particle in particles.iter_mut() {
    //         let a = &mut *self;
    //         let b = &mut *particle;

    //         let normal = a.pos - b.pos;
    //         let unit_normal = normal.normalize();
    //         let unit_tangent = Vec2::new(unit_normal.y, unit_normal.x);
    //         let a_normal = a.vel.dot(unit_normal);
    //         let a_tangent = a.vel.dot(unit_tangent);
    //         let b_normal = b.vel.dot(unit_normal);
    //         let b_tangent = b.vel.dot(unit_tangent);

    //         let a_normal_new =
    //             (a_normal * (a.mass - b.mass) + 2. * b.mass * b_normal) / (a.mass + b.mass);
    //         let b_normal_new =
    //             (b_normal * (b.mass - a.mass) + 2. * a.mass * a_normal) / (a.mass + b.mass);

    //         let a_normal_new = a_normal_new * unit_normal;
    //         let a_tangent_new = a_tangent * unit_tangent;
    //         let b_normal_new = b_normal_new * unit_normal;
    //         let b_tangent_new = b_tangent * unit_tangent;

    //         self.vel.x = a_normal_new.x + a_tangent_new.x;
    //         self.vel.y = a_normal_new.y + a_tangent_new.y;
    //         particle.vel.x = b_normal_new.x + b_tangent_new.x;
    //         particle.vel.y = b_normal_new.y + b_tangent_new.y;
    //     }
    // }
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
            update_tree_when_new_particle(&mut tree, &particles);
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
        update_tree_aabbs(&mut tree, &particles);
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
