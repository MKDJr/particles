// //TODO (Change this to use https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/);
// fn surface_area(tree: Tree) -> f32 {
//     let mut surface_area = 0.;
//     for node in tree.iter() {
//         surface_area += node.bbox.area
//     }
//     return surface_area;
// }

// fn add_particle_to_tree

// fn regenerate_tree(tree: &Vec<Node>, particles: &Vec<Particle>) -> Vec<Node> {
//     let mut new_tree: Vec<Node> = Vec::new();
//     // find root
//     let root_index = tree
//         .iter()
//         .enumerate()
//         .max_by(|(_, a), (_, b)| area(a.aabb).partial_cmp(&area(b.aabb)).unwrap())
//         .unwrap()
//         .0;

//     // add root to stack
//     let mut stack = vec![root_index];

//     /*
//     important caveat

//     i am creating a new tree here, so for each node we go
//     through in the old tree, we must add one to the new tree
//     */
//     // iterate through stack
//     while !stack.is_empty() {
//         let current_node_index = stack.pop().unwrap();

//         match &tree[current_node_index].kind {
//             // if root
//             NodeKind::Root {
//                 indeces_of_particle_group,
//                 left_child_index: _,
//                 right_child_index: _,
//             } => {
//                 // get particles here
//                 let mut working_particles = get_vector_subset(particles, indeces_of_particle_group);

//                 // CHECK IF PARTICLE# IS 1, IF SO, DONE

//                 // split particles at median
//                 working_particles.sort_by(|a, b| b.pos.x.partial_cmp(&a.pos.x).unwrap());
//                 let median_index = working_particles.len() / 2;

//                 // calculate area of aabb for both halves
//                 let (a_group, b_group) = working_particles.split_at(median_index);

//                 // CHECK IF A GROUP IS 1, IF SO MAKE LEAF

//                 // CHEK IF B GROUP IS 1, IF SO MAKE LEAF

//                 let a_group_aabb = create_aabb_particle_group(&a_group.to_vec());
//                 let b_group_aabb = create_aabb_particle_group(&b_group.to_vec());

//                 let a_area = area(a_group_aabb);
//                 let b_area = area(b_group_aabb);
//                 if a_area > b_area {
//                     let left_node = Node {
//                         aabb: b_group_aabb,
//                         kind: NodeKind::InternalNode {
//                             parent_index: (),
//                             indeces_of_particle_group: (),
//                             left_child_index: (),
//                             right_child_index: (),
//                         },
//                     };
//                 }

//                 //             smaller half becomes left child, push to stack as node if #particles > 1, else leaf
//                 //             bigger half becomes right child, push to stack as node if #particles > 1, else leaf
//             }

//             //     if node
//             NodeKind::InternalNode {
//                 parent_index,
//                 indeces_of_particle_group,
//                 left_child_index,
//                 right_child_index,
//             } => {
//                 //         split particles at median
//                 //             calculate aabb for both halves
//                 //             smaller half becomes left child, push to stack as node if #particles > 1, else leaf
//                 //             bigger half becomes right child, push to stack as node if #particles > 1, else leaf
//             }
//             //     if leaf
//             NodeKind::Leaf {
//                 parent_index,
//                 particle_index,
//             } => {}
//         }
//     }
//     return new_tree;
// }

// // TODO have to also be able to move these guys around the tree to reorganize
// fn update_tree_aabbs(tree: &mut Vec<Node>, particles: &Vec<Particle>) {
//     // let mut stack: Vec<usize> = (0..tree.len()).collect();
//     // let mut current_node_kind = NodeKind::Leaf {
//     //     parent_index: 0,
//     //     particle_index: 0,
//     // };

//     let root_index = tree
//         .iter()
//         .enumerate()
//         .max_by(|(_, a), (_, b)| area(a.aabb).partial_cmp(&area(b.aabb)).unwrap())
//         .unwrap()
//         .0;
//     let mut stack = vec![root_index];
//     while !stack.is_empty() {
//         let current_node_index = stack.pop().unwrap();
//         let current_node_kind = tree[current_node_index].kind.clone();
//         match current_node_kind {
//             NodeKind::Leaf {
//                 parent_index: _,
//                 particle_index,
//             } => tree[current_node_index].aabb = create_aabb_particle(&particles[particle_index]),
//             NodeKind::InternalNode {
//                 parent_index: _,
//                 indeces_of_particle_group,
//                 left_child_index: _,
//                 right_child_index: _,
//             } => {
//                 let mut particle_group = Vec::new();
//                 for index in indeces_of_particle_group.iter() {
//                     particle_group.push(particles[*index])
//                 }
//                 tree[current_node_index].aabb = create_aabb_particle_group(&particle_group)

//                 // calculate new aabb for node
//                 // calculate new area for left child
//                 // calculage new area for right child
//                 // calculate max of these areas
//                 // associated node replaces this node
//                 // former this node is demoted to where the child node was
//             }
//             NodeKind::Root {
//                 indeces_of_particle_group,
//                 left_child_index,
//                 right_child_index,
//             } => {
//                 let mut particle_group = Vec::new();
//                 for index in indeces_of_particle_group.iter() {
//                     particle_group.push(particles[*index])
//                 }
//                 tree[current_node_index].aabb = create_aabb_particle_group(&particle_group);
//                 if let Some(left_child_index) = left_child_index {
//                     stack.push(left_child_index)
//                 };
//                 if let Some(right_child_index) = right_child_index {
//                     stack.push(right_child_index)
//                 };
//             }
//         }
//     }
// }

// fn update_tree_when_new_particle(tree: &mut Vec<Node>, particles: &Vec<Particle>) {
//     let now = Instant::now();
//     // let mut tree_len = 0usize;

//     let root_index = tree
//         .iter()
//         .enumerate()
//         .max_by(|(_, a), (_, b)| area(a.aabb).partial_cmp(&area(b.aabb)).unwrap())
//         .unwrap()
//         .0;

//     let mut stack = vec![root_index]; //put root index in there

//     let new_particle = particles.last().unwrap();
//     let new_particle_bbox = create_aabb_particle(new_particle);
//     let new_particle_index = particles.iter().position(|p| p == new_particle);

//     while !stack.is_empty() {
//         let current_node_index = stack.pop().unwrap();

//         let mut current_node_kind = tree[current_node_index].kind.clone();
//         // what type of node it is
//         match current_node_kind {
//             NodeKind::Leaf {
//                 ref mut parent_index,
//                 particle_index,
//             } => {
//                 // create new node and make it's leaves the new leaves

//                 let new_internal_node = Node {
//                     aabb: union(new_particle_bbox, tree[current_node_index].aabb),
//                     kind: NodeKind::InternalNode {
//                         parent_index: (*parent_index),
//                         indeces_of_particle_group: (vec![
//                             particle_index,
//                             new_particle_index.unwrap(),
//                         ]),
//                         left_child_index: tree
//                             .iter()
//                             .position(|n| n == &tree[current_node_index])
//                             .unwrap(),
//                         right_child_index: tree.len() + 1,
//                     },
//                 };
//                 tree.push(new_internal_node);

//                 // make this leaf's parent the new node
//                 *parent_index = tree.iter().position(|n| n == tree.last().unwrap()).unwrap();

//                 // create leaf from new particle

//                 let new_leaf = Node {
//                     aabb: new_particle_bbox,
//                     kind: NodeKind::Leaf {
//                         parent_index: tree.iter().position(|n| n == tree.last().unwrap()).unwrap(),
//                         particle_index: new_particle_index.unwrap(),
//                     },
//                 };
//                 // new_leaf_copy = new_leaf.clone();
//                 tree.push(new_leaf);
//             }
//             NodeKind::InternalNode {
//                 parent_index: _,
//                 mut indeces_of_particle_group,
//                 left_child_index,
//                 right_child_index,
//             } => {
//                 indeces_of_particle_group.push(new_particle_index.unwrap());
//                 //calculate areas
//                 let left_theoretical_area =
//                     area(union(new_particle_bbox, tree[left_child_index].aabb));
//                 println!("left_theoretical_area: {}", left_theoretical_area);
//                 let right_theoretical_area =
//                     area(union(new_particle_bbox, tree[right_child_index].aabb));
//                 println!("right_theoretical_area: {}", right_theoretical_area);

//                 // compare areas
//                 if left_theoretical_area > right_theoretical_area {
//                     stack.push(right_child_index);
//                     println!("move to right child");
//                 } else {
//                     stack.push(left_child_index);
//                     println!("move to left child");
//                 }
//             }
//             NodeKind::Root {
//                 mut indeces_of_particle_group,
//                 ref mut left_child_index,
//                 ref mut right_child_index,
//             } => {
//                 // if root, add new particle to indeces stored by root
//                 indeces_of_particle_group.push(new_particle_index.unwrap());

//                 //
//                 if let (Some(left_child), Some(right_child)) =
//                     (*left_child_index, *right_child_index)
//                 {
//                     let left_theoretical_area =
//                         area(union(new_particle_bbox, tree[left_child].aabb));
//                     let right_theoretical_area =
//                         area(union(new_particle_bbox, tree[right_child].aabb));

//                     // compare areas
//                     if left_theoretical_area > right_theoretical_area {
//                         stack.push(right_child);
//                     } else {
//                         stack.push(left_child)
//                     }
//                 } else if let Some(left_child) = left_child_index {
//                     *right_child_index = new_particle_index;
//                     let new_leaf = Node {
//                         aabb: new_particle_bbox,
//                         kind: NodeKind::Leaf {
//                             parent_index: current_node_index,
//                             particle_index: new_particle_index.unwrap(),
//                         },
//                     };
//                     tree.push(new_leaf)
//                 } else {
//                     *left_child_index = new_particle_index;
//                     let new_leaf = Node {
//                         aabb: new_particle_bbox,
//                         kind: NodeKind::Leaf {
//                             parent_index: current_node_index,
//                             particle_index: new_particle_index.unwrap(),
//                         },
//                     };
//                     tree.push(new_leaf)
//                 }
//             }
//         }
//     }
//     println!(
//         "{} ns to add new particle to tree",
//         now.elapsed().as_nanos()
//     )
// }

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