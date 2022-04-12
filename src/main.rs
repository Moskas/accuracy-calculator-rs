use std::env;
use std::io::{self, Write};

fn read_judgments() -> Vec<i32> {
    println!("Please insert your judgments");
    let possible_judgments = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut judgements: Vec<i32> = Vec::new();
    let mut iterator: i32 = 0;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgments[iterator as usize]);
        io::stdout().flush();
        match io::stdin().read_line(&mut readout) {
            Ok(_) => {
                match readout.trim().parse::<i32>() {
                    Ok(o) => {
                        judgements.push(o);
                        iterator += 1
                    }
                    Err(e) => {
                        println!("Incorrect value! Blad: {}", e);
                    }
                };
            }
            Err(err) => println!("Couldn't read the value: {}", err),
        }
        readout.clear();
    }
    return judgements;
}

fn perfect_all(judgments: &Vec<i32>) -> (f32, f32) {
    let perfects: i32 = judgments[0..=1].iter().sum();
    let other: i32 = judgments[2..=5].iter().sum();
    return (perfects as f32, other as f32);
}

fn percent_v1(judgements: &Vec<i32>) -> f32 {
    let percent: (i32, i32) = (
        300 * (judgements[0] + judgements[1])
            + 200 * judgements[2]
            + 100 * judgements[3]
            + 50 * judgements[4],
        (judgements[0..=5].iter().sum::<i32>()) * 300,
    );
    return (percent.0 as f32 / percent.1 as f32) * 100.0;
}

fn percent_v2(judgements: &Vec<i32>) -> f32 {
    let percent: (i32, i32) = (
        305 * judgements[0]
            + 300 * judgements[1]
            + 200 * judgements[2]
            + 100 * judgements[3]
            + 50 * judgements[4],
        (judgements[0..=5].iter().sum::<i32>()) * 305,
    );
    return (percent.0 as f32 / percent.1 as f32) * 100.0;
}

fn grade(percent: f32) -> String {
    match percent {
        100.0 => return "SS".to_string(),
        95.01..=100.0 => return "S".to_string(),
        90.01..=95.0 => return "A".to_string(),
        80.01..=90.0 => return "B".to_string(),
        70.01..=80.0 => return "C".to_string(),
        _ => return "D".to_string(),
    };
}

fn calculate() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let query = &args[1];
        match query.as_str() {
            "-h" => println!("Here will be help format"), // TODO  Proper help command
            "-v" => println!("Version 0.0.1"),
            "-j" => {
                let arg_input = args[2].split(',');
                let mut judgements: Vec<&str> = arg_input.collect();
                while judgements.len() < 6 {
                    judgements.push("0"); //  Fill missing spaces with 0 TODO fillout prompt
                }
                let mut judgements_i32: Vec<i32> = Vec::new();
                for val in judgements {
                    judgements_i32.push(val.parse::<i32>().unwrap());
                }
                println!("{:?}", judgements_i32);
            }
            _ => (), //  Do nothing in match, go to the else statement
                     //  TODO refactor logic to be here rather than in else block
        }
    } else {
        let judgements = read_judgments();
        let ma: f32 = judgements[0] as f32 / judgements[1] as f32;
        let judge_sum_tuple: (f32, f32) = perfect_all(&judgements);
        let pa: f32 = judge_sum_tuple.0 / judge_sum_tuple.1;
        println!("Your MA is: {} ({}:{})", ma, judgements[0], judgements[1]);
        println!(
            "Your PA is: {} ({}:{})",
            pa, judge_sum_tuple.0, judge_sum_tuple.1
        );
        println!(
            "Acc V1: {}% Grade: {}",
            percent_v1(&judgements),
            grade(percent_v1(&judgements))
        );
        println!(
            "Acc V2: {}% Grade: {}",
            percent_v2(&judgements),
            grade(percent_v2(&judgements))
        );
    }
}
