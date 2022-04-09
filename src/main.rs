use std::io;

fn read_judgments() -> Vec<i32> {
    let mut judgements: Vec<i32> = Vec::new();
    let mut iterator: i32 = 0;
    let mut readout: String = String::new();
    while iterator < 6 {
        io::stdin()
            .read_line(&mut readout)
            .expect("Failed to read line");
        match readout.trim().parse::<i32>() {
            Ok(o) => {
                judgements.push(o);
                iterator += 1
            }
            Err(e) => {
                println!("Podales zla wartosc! Blad: {}", e);
                iterator -= 1
            }
        };
        readout.clear();
    }
    return judgements;
}

fn main() {
    let judgements: Vec<i32> = read_judgments();
    let ratio: f32 = judgements[0] as f32 / judgements[1] as f32;
    println!("{:?}", judgements);
    println!(
        "Twoje MA wynosi: {} ({}:{})",
        ratio, judgements[0], judgements[1]
    );
    println!("Twoje PA wynosi: {}", "chuj");
}
