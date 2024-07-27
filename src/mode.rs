use crate::input_functions;
use std::collections::HashMap;

pub fn mode() {
    let numbers: Vec<i32> = input_functions::request_read_numbers();

    if let Some(mode) = get_mode(&numbers) {
        println!("The mode is: {}", mode);
    } else {
        println!("No mode found");
    }
}

fn get_mode(array: &[i32]) -> Option<i32> {
    if array.is_empty() {
        return None;
    }

    if array.len() == 1 {
        return Some(array[0]);
    }

    let mut hash_map: HashMap<i32,i32> = HashMap::new();

    let mut max_val = array[0];
    let mut max_count = 1;

    for num in array.iter().copied() {
        let count = hash_map.entry(num).and_modify(|n|{ *n += 1 }).or_insert(1);
        if *count > max_count {
            max_count = *count;
            max_val = num;
        }
    }

    Some(max_val)
}