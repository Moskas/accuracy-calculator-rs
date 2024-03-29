use std::io::{self, Write};

pub fn read_judgements() -> Vec<i32> {
    println!("Please insert your judgements");
    let possible_judgements = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut judgements: Vec<i32> = Vec::with_capacity(6);
    let mut iterator: i32 = 0;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgements[iterator as usize]);
        io::stdout().flush().unwrap(); //  Making sure judgment name shows up before reading input
        match io::stdin().read_line(&mut readout) {
            Ok(_) => {
                match readout.trim().parse::<i32>() {
                    Ok(o) => match o {
                        o if 0 <= o => {
                            judgements.push(o);
                            iterator += 1
                        }
                        _ => println!("Value can't be less than 0"),
                    },
                    Err(err) => {
                        println!("Incorrect value Error: {}", err);
                    }
                };
            }
            Err(err) => println!("Couldn't read the value Error: {}", err),
        }
        readout.clear();
    }
    judgements
}
/*
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

#[test]
fn fill_test() {
    let mut vec_test: Vec<i32> = vec![300, 200];
    assert_eq!(
        fill(vec_test.len(), &mut vec_test),
        vec![300, 200, 0, 0, 0, 0]
    );
    let mut vec_test: Vec<i32> = Vec::new();
    assert_eq!(fill(vec_test.len(), &mut vec_test), vec![0, 0, 0, 0, 0, 0]);
    let mut vec_test: Vec<i32> = vec![300, 200, 29, 29, 1, 1];
    assert_eq!(
        fill(vec_test.len(), &mut vec_test),
        vec![300, 200, 29, 29, 1, 1]
    );
}
*/
/*
pub fn read_missing(judgements: &mut Vec<i32>) -> Vec<i32> {
    println!("Please insert your judgements");
    let possible_judgements = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut iterator: i32 = judgements.len() as i32;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgements[iterator as usize]);
        io::stdout().flush().unwrap(); //  Making sure judgment name shows up before reading input
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
*/
//  Converting from Vector with &str to i32 type
pub fn convert_to_i32(judgements: &mut Vec<&str>) -> Vec<i32> {
    let mut judgements_i32: Vec<i32> = Vec::new();
    for val in judgements {
        match val.parse::<i32>() {
            Ok(val) => {
                if val < 0 {
                    println!("Value can't be less than 0");
                    judgements_i32.push(0o0)
                } else {
                    judgements_i32.push(val)
                }
            }
            Err(err) => {
                println!("Please enter only numbers Error: {}", err);
                break;
            }
        }
    }
    judgements_i32
}

#[test]
fn convert_to_i32_test() {
    let mut vec_test: Vec<&str> = vec!["300", "200", "100", "50", "0", "0"];
    assert_eq!(convert_to_i32(&mut vec_test), vec![300, 200, 100, 50, 0, 0]);
    let mut vec_test: Vec<&str> = vec!["-1", "200", "100", "50", "0", "0"];
    assert_eq!(convert_to_i32(&mut vec_test), vec![0, 200, 100, 50, 0, 0]);
}
/*
pub fn read_arguments(args: &Vec<String>) {
    for i in 1..args.len() as i32 {
        match args[i as usize].as_str() {
            "-h" => println!("Available arguments:\n-h - prints out help\n-v - prints out version\n-j - pass judgements in format marv,perf,great,good,bad,miss"),
            "-v" => println!("Version 0.4.6"),
            "-j" => {
                if args.len() > (i+1) as usize{
                let arg_input = args[(i + 1) as usize].split(',');
                let mut judgements: Vec<&str> = arg_input.collect();
                let mut judgements_i32 = convert_to_i32(&mut judgements);
                    for i in &judgements_i32{
                        if i<&0{
                            println!("Judgement values can't be lower than 0!: {}",i);
                            std::process::exit(*i);
                            }
                        }
                println!("Judgements read on launch {:?}", judgements_i32);
                if judgements_i32.len() != 6 {
                    fill(judgements_i32.len(),&mut judgements_i32);
                    let result = crate::calculate::calculate(&judgements_i32);
                    crate::print::print_out(result,crate::calculate::percent_v1(&judgements_i32),crate::calculate::percent_v2(&judgements_i32));

                    } else {
                    let result = crate::calculate::calculate(&judgements_i32);
                    crate::print::print_out(result,crate::calculate::percent_v1(&judgements_i32),crate::calculate::percent_v2(&judgements_i32));
                    }
                } else {
                    println!("No judgments were passed after -j");
                }
            },
            "-w" => {
                let judgemnets = crate::read::read_judgements();
                let result = crate::calculate::calculate(&judgemnets);
                crate::write_result::write(result).unwrap(); // Writing out the result into formatted text file WIP
            },
            _ => {}, //  Print out in case of usage of other letter than v,j,h
            }
    }
}*/
