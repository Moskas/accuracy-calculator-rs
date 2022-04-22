use std::{env, io};

mod calculate;
mod read;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() as i32 {
        // if no argument is provided do manual input
        1 => {
            let result = calculate::calculate(read::read_judgments());
            println!("Your MA is: {} ({}:{})", result.0, result.3[0], result.3[1]);
            println!("Your PA is: {} ({}:{})", result.1, result.2 .0, result.2 .1);
            println!(
                "Acc V1: {}% Grade: {}",
                calculate::percent_v1(&result.3),
                calculate::grade(calculate::percent_v1(&result.3))
            );
            println!(
                "Acc V2: {}% Grade: {}",
                calculate::percent_v2(&result.3),
                calculate::grade(calculate::percent_v2(&result.3))
            );
        }
        // else read argument and do instruction from other arms
        _ => {
            let query = &args[1];
            match query.as_str() {
            "-h" => println!("Available arguments:\n-h - prints out help\n-v - prints out version\n-j - pass judgements in format marv,perf,great,good,bad,miss"),
            "-v" => println!("Version 0.3"),
            "-j" => {
                let arg_input = args[2].split(',');
                let mut judgements: Vec<&str> = arg_input.collect();
//                while judgements.len() < 6 {
//                    judgements.push("0"); //  Fill missing spaces with 0 TODO optional fillout prompt
//                }
                let mut judgements_i32: Vec<i32> = Vec::new();
                for val in judgements {
                   match val.parse::<i32>() {
                       Ok(val) => judgements_i32.push(val),
                       Err(err) => {
                           println!("Please enter only numbers Error: {}", err);
                           break
                       },
                   }
                }
                println!("{:?}", judgements_i32);
                if judgements_i32.len() != 6 {
                    read::fill(judgements_i32.len(),&mut judgements_i32);
                    let result = calculate::calculate(judgements_i32);
                println!("Your MA is: {} ({}:{})", result.0, result.3[0], result.3[1]);
                println!("Your PA is: {} ({}:{})", result.1, result.2 .0, result.2 .1);
                println!(
                    "Acc V1: {}% Grade: {}",
                    calculate::percent_v1(&result.3),
                    calculate::grade(calculate::percent_v1(&result.3))
                );
                println!(
                    "Acc V2: {}% Grade: {}",
                    calculate::percent_v2(&result.3),
                    calculate::grade(calculate::percent_v2(&result.3)));
                } else {
                let result = calculate::calculate(judgements_i32);
                println!("Your MA is: {} ({}:{})", result.0, result.3[0], result.3[1]);
                println!("Your PA is: {} ({}:{})", result.1, result.2 .0, result.2 .1);
                println!(
                    "Acc V1: {}% Grade: {}",
                    calculate::percent_v1(&result.3),
                    calculate::grade(calculate::percent_v1(&result.3))
                );
                println!(
                    "Acc V2: {}% Grade: {}",
                    calculate::percent_v2(&result.3),
                    calculate::grade(calculate::percent_v2(&result.3)));
                }
            }
            _ => {println!("Wrong argument");}, //  Print out in case of usage of other letter than v,j,h
            }
        }
    }
}
