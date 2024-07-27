use crate::input_functions;

pub fn median() {
    let numbers: Vec<i32> = input_functions::request_read_numbers();

    if let Some(median) = get_median(&numbers) {
            println!("The median is: {}", median);
    } else {
        println!("No median found");
    }
}

fn get_median(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }

    let mid = v.len() / 2;
    
    if mid == 0 {
        return Some(v[0] as f64);
    }

    let mut v = v.to_vec();
    v.sort();

    if (mid % 2) == 0 {
        return Some((v[mid] + v[mid - 1]) as f64 / 2.0);
    } else {
        return Some(v[mid] as f64);
    }
}