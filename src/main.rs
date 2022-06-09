use clap::{App, Arg};
use std::env;

mod calculate;
mod print;
mod read;
mod write_result;

fn main() {
    //let args: Vec<String> = env::args().collect();
    /*match args.len() as i32 {
        // if no argument is provided do manual input
        1 => {
            let judgements = read::read_judgements();
            let result = calculate::calculate(&judgements);
            let score_v1 = calculate::percent_v1(&judgements);
            let score_v2 = calculate::percent_v2(&judgements);
            print::print_out(result, score_v1, score_v2);
        }
        // else read argument and do instruction from other arms
        _ => {
            read::read_arguments(&args);
        }
    }*/
    let matches = App::new("accuracy-calculator")
        .version("0.4.6")
        .author("Moskas")
        .about("osu!mania CLI accuracy calculator written in Rust")
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("launch app in interactive mode")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("judgments")
                .short("j")
                .help("Pass judgments in format 300g,300,200,100,50,miss")
                .takes_value(true),
        )
        .get_matches();
    if matches.is_present("interactive") {
        let judgements = read::read_judgements();
        let result = calculate::calculate(&judgements);
        let score_v1 = calculate::percent_v1(&judgements);
        let score_v2 = calculate::percent_v2(&judgements);
        print::print_out(result, score_v1, score_v2)
    } else if matches.is_present("judgments") {
        println!("{}", matches.value_of_lossy("judgments").unwrap());
    }
}
