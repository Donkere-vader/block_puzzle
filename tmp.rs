fn rotate_coordinates_right(original_x: usize, original_y: usize, matrix_size: usize) -> (usize, usize) {
    let rotated_x = original_y;
    let rotated_y = matrix_size - 1 - original_x;
    (rotated_x, rotated_y)
}

fn rotate_coordinates_left(original_x: usize, original_y: usize, matrix_size: usize) -> (usize, usize) {
    let rotated_y = original_x;
    let rotated_x = matrix_size - 1 - original_y;
    (rotated_x, rotated_y)
}

fn main() {
    let matrix_size = 4; // Change this to the size of your matrix

    let original_x = 1; // Original x-coordinate
    let original_y = 0; // Original y-coordinate

    let (rotated_right_x, rotated_right_y) = rotate_coordinates_right(original_x, original_y, matrix_size);
    let (rotated_left_x, rotated_left_y) = rotate_coordinates_left(original_x, original_y, matrix_size);

    println!("Original coordinates: ({}, {})", original_x, original_y);
    println!("Rotated left coordinates: ({}, {})", rotated_left_x, rotated_left_y);
    println!("Rotated right coordinates: ({}, {})", rotated_right_x, rotated_right_y);
}