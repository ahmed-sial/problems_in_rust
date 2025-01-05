fn linear_search(arr: &[i32], to_find: i32) -> Option<usize> {
    for (index, value) in arr.iter().enumerate() {
        if *value == to_find {
            return Some(index);
        }
    }
    None
}
