pub fn get_index_matrix(width: usize, given_location: &crate::BasicVector<usize>) -> usize {
    given_location.y * width + given_location.x
}
