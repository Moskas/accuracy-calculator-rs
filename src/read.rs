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
