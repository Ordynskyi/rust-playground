use std::io::{self, ErrorKind};
use crate::Operation;

pub fn request_enter_operation() -> Result<Operation, io::Error> {
    println!("Select the operation:");
    println!("1. Find median");
    println!("2. Find mode");
    println!("3. Pig Latin");
    println!("4. Quit");

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let choice = input.trim().parse::<i32>()
        .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Enter a valid number"))?;
    
    match choice {
        1 => Ok(Operation::Median),
        2 => Ok(Operation::Mode),
        3 => Ok(Operation::PigLatin),
        4 => Ok(Operation::Quit),
        _ => Err(io::Error::new(ErrorKind::InvalidInput, "Enter a valid number")),
    }
}

pub fn request_read_numbers() -> Vec<i32> {
    println!("Enter the numbers separated by commas:");
    loop {
        let mut nums_input = String::new();
        
        let nums_input = match io::stdin().read_line(&mut nums_input) {
            Ok(_) => nums_input,
            Err(e) => {
                println!("Reading input error: {}", e);
                continue;
            }
        };
        
        let nums:Vec<i32> = nums_input.split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        
        if nums.is_empty() {
            println!("No numbers entered. Please try again.");
            continue;
        } 
        
        println!("The numbers entered are: {}", vec_nums_to_string(&nums));
        return nums;        
    }
}

pub fn request_read_string() -> Result<String, io::Error> {
    println!("Enter the string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Result::Ok(input)
}

fn vec_nums_to_string(v: &[i32]) -> String {
    let mut entered_nums = String::new();
    for num in v.iter().copied() {
        entered_nums.push_str(&num.to_string());
        entered_nums.push_str(", ");
    }
    entered_nums.pop();
    entered_nums.pop();
    entered_nums
}