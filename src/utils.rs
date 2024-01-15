fn get_vector_subset(vector: &Vec<Particle>, indeces_to_get: &Vec<usize>) -> Vec<Particle> {
    let mut new_vector = Vec::new();
    for index in indeces_to_get.iter() {
        new_vector.push(vector[*index])
    }
    return new_vector;
}
