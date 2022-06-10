use clap::{App, Arg};

mod calculate;
mod print;
mod read;
mod write_result;

fn main() {
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
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .help("Save result to a text file")
                .takes_value(false),
        )
        .get_matches();
    if matches.is_present("interactive") {
        let judgements = read::read_judgements();
        let result = calculate::calculate(&judgements);
        print::print_out(result)
    } else if matches.is_present("judgments") {
        let judgements: Vec<i32> = read::convert_to_i32(
            &mut matches
                .value_of("judgments")
                .unwrap()
                .split(',')
                .collect::<Vec<&str>>(),
        );
        let result = calculate::calculate(&judgements);
        print::print_out(result);
    }
}
