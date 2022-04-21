use std::io::{self, Write};

pub fn read_judgments() -> Vec<i32> {
    println!("Please insert your judgments");
    let possible_judgments = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut judgements: Vec<i32> = Vec::new();
    let mut iterator: i32 = 0;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgments[iterator as usize]);
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
    println!("Missing some judgments, do automatic fill with 0 or manually input missing judgments?:\ny/n");
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
    println!("Please insert your judgments");
    let possible_judgments = vec!["300g", "300", "200", "100", "50", "Miss"];
    let mut iterator: i32 = judgements.len() as i32;
    let mut readout: String = String::new();
    while iterator < 6 {
        print!("{}: ", possible_judgments[iterator as usize]);
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
