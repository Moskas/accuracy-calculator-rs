use std::env;

mod calculate;
mod read;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() as i32 {
        // if no argument is provided do manual input
        1 => {
            let result = calculate::calculate(read::read_judgments());
            calculate::print_out(result);
        }
        // else read argument and do instruction from other arms
        _ => {
            let query = &args[1];
            read::read_arguments(query, &args);
        }
    }
}
