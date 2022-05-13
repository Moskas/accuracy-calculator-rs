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
        percent if percent == 100.0 => return "SS".to_string(),
        percent if (95.00001..=100.0).contains(&percent) => return "S".to_string(),
        percent if (90.00001..=95.00).contains(&percent) => return "A".to_string(),
        percent if (80.00001..=90.0).contains(&percent) => return "B".to_string(),
        percent if (70.00001..=80.0).contains(&percent) => return "C".to_string(),
        _ => return "D".to_string(),
    };
}

#[test]
fn grade_test() {
    assert_eq!(grade(70.00), "D");
    assert_eq!(grade(80.00), "C");
    assert_eq!(grade(90.00), "B");
    assert_eq!(grade(90.01), "A");
    assert_eq!(grade(95.00), "A");
    assert_eq!(grade(95.01), "S");
}

//  Calculating ratio of 300g to 300, 300g+300 to other judgements and returning everything as a tuple
pub fn calculate(judgements: &Vec<i32>) -> (f32, f32, (f32, f32), &Vec<i32>) {
    let ma: f32 = judgements[0] as f32 / judgements[1] as f32;
    let judge_sum_tuple: (f32, f32) = perfect_all(&judgements);
    let pa: f32 = judge_sum_tuple.0 / judge_sum_tuple.1;
    return (ma, pa, judge_sum_tuple, judgements);
}
