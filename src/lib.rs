pub fn rotate_left_in_place(d: usize, arr: &mut [i32]) {
    let n = arr.len();
    if n == 0 || d % n == 0 {
        return;
    }

    // Reverse the array into 3 steps: first part, second part, and whole array.
    arr.reverse();
    arr[..n - d].reverse();
    arr[n - d..].reverse();
}
