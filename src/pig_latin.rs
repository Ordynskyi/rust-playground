use crate::input_functions;

pub fn pig_latin() {

    let input: String;
    loop {
        match input_functions::request_read_string() {
            Ok(str) => {
                input = str;
                break;
            },
            Err(e) => {
                println!("{e}. Please try again.");
                continue;
            },  
        };
    }

    let converted_str = get_converted_str(&input);
    println!("Pig latin string is: '{converted_str}'");
}

fn get_converted_str(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    let mut result = String::with_capacity(input.len() * 2);
    
    for word in input.split_whitespace() {
        let mut chars = word.chars();

        if let Some(first_char) = chars.next() {
            let pig_latin_word = if vowels.contains(&first_char) {
                format!("{}-hay ", word)
            } else {
                format!("{}-{}ay ", chars.as_str(), first_char)
            };
            
            result.push_str(&pig_latin_word);
        }
    }

    if !result.is_empty() {
        result.pop();
    }

    return result;
}