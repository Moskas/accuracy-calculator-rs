use std::io::{self, Write};

pub fn read_judgements() -> Vec<i32> {
    println!("Please insert your judgements");
    let possible_judgements = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut judgements: Vec<i32> = Vec::new();
    let mut iterator: i32 = 0;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgements[iterator as usize]);
        io::stdout().flush(); //  Making sure judgment name shows up before reading input
        match io::stdin().read_line(&mut readout) {
            Ok(_) => {
                match readout.trim().parse::<i32>() {
                    Ok(o) => {
                        judgements.push(o);
                        iterator += 1
                    }
                    Err(err) => {
                        println!("Incorrect value Error: {}", err);
                    }
                };
            }
            Err(err) => println!("Couldn't read the value Error: {}", err),
        }
        readout.clear();
    }
    return judgements;
}

pub fn fill(size: usize, judgements: &mut Vec<i32>) -> Vec<i32> {
    println!("Missing some judgements, do automatic fill with 0 or manually input missing judgements?:\ny/n");
    let mut readout: String = String::new();
    for _i in (judgements.len() as i32)..=6 {
        match io::stdin().read_line(&mut readout) {
            Ok(_) => match readout.trim() {
                "y" => {
                    for _i in size..6 {
                        judgements.push(0);
                    }
                    return judgements.to_vec();
                }
                "n" => {
                    read_missing(judgements);
                    return judgements.to_vec();
                }
                _ => {
                    println!("Incorrect option");
                    return judgements.to_vec();
                }
            },
            Err(err) => {
                println!("Incorrect value Error: {}", err);
                return judgements.to_vec();
            }
        }
    }
    return judgements.to_vec();
}

pub fn read_missing(judgements: &mut Vec<i32>) -> Vec<i32> {
    println!("Please insert your judgements");
    let possible_judgements = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut iterator: i32 = judgements.len() as i32;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgements[iterator as usize]);
        io::stdout().flush(); //  Making sure judgment name shows up before reading input
        match io::stdin().read_line(&mut readout) {
            Ok(_) => {
                match readout.trim().parse::<i32>() {
                    Ok(o) => {
                        judgements.push(o);
                        iterator += 1
                    }
                    Err(err) => {
                        println!("Incorrect value Error: {}", err);
                    }
                };
            }
            Err(err) => println!("Couldn't read the value Error: {}", err),
        }
        readout.clear();
    }
    return judgements.to_vec();
}
//  Converting from Vector with &str to i32 type
pub fn convert_to_i32(judgements: &mut Vec<&str>) -> Vec<i32> {
    let mut judgements_i32: Vec<i32> = Vec::new();
    for val in judgements {
        match val.parse::<i32>() {
            Ok(val) => judgements_i32.push(val),
            Err(err) => {
                println!("Please enter only numbers Error: {}", err);
                break;
            }
        }
    }
    return judgements_i32;
}

pub fn read_arguments(args: &Vec<String>) {
    for i in 1..args.len() as i32 {
        match args[i as usize].as_str() {
            "-h" => println!("Available arguments:\n-h - prints out help\n-v - prints out version\n-j - pass judgements in format marv,perf,great,good,bad,miss"),
            "-v" => println!("Version 0.4.5"),
            "-j" => {
                if args.len() > (i+1) as usize{
                let arg_input = args[(i + 1) as usize].split(',');
                let mut judgements: Vec<&str> = arg_input.collect();
                let mut judgements_i32 = convert_to_i32(&mut judgements);
                println!("Judgements read on launch {:?}", judgements_i32);
                if judgements_i32.len() != 6 {
                    fill(judgements_i32.len(),&mut judgements_i32);
                    let result = crate::calculate::calculate(judgements_i32);
                    crate::calculate::print_out(result);
                    } else {
                    let result = crate::calculate::calculate(judgements_i32);
                    crate::calculate::print_out(result);
                    }
                } else {
                    println!("No judgments were passed after -j");
                }
            }
            _ => {}, //  Print out in case of usage of other letter than v,j,h
            }
    }
}
