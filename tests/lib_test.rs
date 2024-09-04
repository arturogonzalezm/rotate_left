use rotate_left::rotate_left_in_place;

#[test]
fn test_rotate_left_in_place_no_rotation() {
    let mut arr = vec![1, 2, 3, 4, 5];
    rotate_left_in_place(0, &mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]); // Expect no change if d = 0
}

#[test]
fn test_rotate_left_in_place_one_rotation() {
    let mut arr = vec![1, 2, 3, 4, 5];
    rotate_left_in_place(1, &mut arr);
    assert_eq!(arr, vec![2, 3, 4, 5, 1]); // Expect the array to rotate by 1 step
}

#[test]
fn test_rotate_left_in_place_multiple_rotations() {
    let mut arr = vec![1, 2, 3, 4, 5];
    rotate_left_in_place(2, &mut arr);
    assert_eq!(arr, vec![3, 4, 5, 1, 2]); // Expect the array to rotate by 2 steps
}

#[test]
fn test_rotate_left_in_place_empty_array() {
    let mut arr: Vec<i32> = vec![];
    rotate_left_in_place(3, &mut arr); // Rotating an empty array should not change it
    assert_eq!(arr, vec![]); // Expect the array to remain empty
}

#[test]
fn test_rotate_left_in_place_full_rotation() {
    let mut arr = vec![1, 2, 3, 4, 5];
    rotate_left_in_place(5, &mut arr); // Rotating by the length of the array should return the same array
    assert_eq!(arr, vec![1, 2, 3, 4, 5]); // Expect no change
}
