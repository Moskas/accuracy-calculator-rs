use std::io::{self, Write};

fn read_judgments() -> Vec<i32> {
    println!("Podaj swoje trafienia");
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
                        println!("Podales zla wartosc! Blad: {}", e);
                        iterator -= 1
                    }
                };
            }
            Err(err) => println!("Nie mozna odczytac wartosci: {}", err),
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

fn main() {
    let judgements: Vec<i32> = read_judgments();
    let ma: f32 = judgements[0] as f32 / judgements[1] as f32;
    let judge_sum_tuple: (f32, f32) = perfect_all(&judgements);
    let pa: f32 = judge_sum_tuple.0 / judge_sum_tuple.1;
    println!(
        "Twoje MA wynosi: {} ({}:{})",
        ma, judgements[0], judgements[1]
    );
    println!(
        "Twoje PA wynosi: {} ({}:{})",
        pa, judge_sum_tuple.0, judge_sum_tuple.1
    );
    println!("Acc V1: {}%", percent_v1(&judgements));
    println!("Acc V2: {}%", percent_v2(&judgements));
}
