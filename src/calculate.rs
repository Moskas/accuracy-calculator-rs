//  Adding up 300g to 300 and other judgements with each other and returning the values as tuple
pub fn perfect_all(judgements: &Vec<i32>) -> (f32, f32) {
    let perfects: i32 = judgements[0..=1].iter().sum();
    let other: i32 = judgements[2..=5].iter().sum();
    return (perfects as f32, other as f32);
}
// Calculating Score v1 percent from formula that can be found here: https://osu.ppy.sh/wiki/en/Gameplay/Accuracy
pub fn percent_v1(judgements: &Vec<i32>) -> f32 {
    let percent: (i32, i32) = (
        300 * (judgements[0] + judgements[1])
            + 200 * judgements[2]
            + 100 * judgements[3]
            + 50 * judgements[4],
        (judgements[0..=5].iter().sum::<i32>()) * 300,
    );
    return (percent.0 as f32 / percent.1 as f32) * 100.0;
}
// Calculating Score v2 percent from formula that can be found here: https://osu.ppy.sh/wiki/en/Gameplay/Accuracy
pub fn percent_v2(judgements: &Vec<i32>) -> f32 {
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
//  Matching Score v1/v2 percanteges with grades https://osu.ppy.sh/wiki/en/Gameplay/Grade
//  wiki page says grades start from .00 in reality next grade starts from .01 due to rounding up
pub fn grade(percent: f32) -> String {
    match percent {
        100.0 => return "SS".to_string(),
        95.01..=100.0 => return "S".to_string(),
        90.01..=95.0 => return "A".to_string(),
        80.01..=90.0 => return "B".to_string(),
        70.01..=80.0 => return "C".to_string(),
        _ => return "D".to_string(),
    };
}
//  Calculating ratio of 300g to 300, 300g+300 to other judgements and returning everything as a tuple
pub fn calculate(judgements: Vec<i32>) -> (f32, f32, (f32, f32), Vec<i32>) {
    let ma: f32 = judgements[0] as f32 / judgements[1] as f32;
    let judge_sum_tuple: (f32, f32) = perfect_all(&judgements);
    let pa: f32 = judge_sum_tuple.0 / judge_sum_tuple.1;
    return (ma, pa, judge_sum_tuple, judgements);
}
//  Printing out results
pub fn print_out(result: (f32, f32, (f32, f32), Vec<i32>)) {
    println!("Your MA is: {} ({}:{})", result.0, result.3[0], result.3[1]);
    println!("Your PA is: {} ({}:{})", result.1, result.2 .0, result.2 .1);
    println!(
        "Acc V1: {}% Grade: {}",
        percent_v1(&result.3),
        grade(percent_v1(&result.3))
    );
    println!(
        "Acc V2: {}% Grade: {}",
        percent_v2(&result.3),
        grade(percent_v2(&result.3))
    );
}
