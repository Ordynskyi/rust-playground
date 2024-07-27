mod input_functions;
mod pig_latin;
mod median;
mod mode;
mod companies_cli;

enum Operation {
    Median,
    Mode,
    PigLatin,
    Quit,
}

fn main() {
    loop {
        let operation = match input_functions::request_enter_operation() {
            Ok(func) => func,
            Err(e) => {
                println!("{e}");
                continue;
            },
        };    

        match operation {
            Operation::Median => median::median(),
            Operation::Mode => mode::mode(),
            Operation::PigLatin => pig_latin::pig_latin(),
            Operation::Quit => break,
        }
    }
}