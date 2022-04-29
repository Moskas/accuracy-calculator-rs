//  Printing out results
pub fn print_out(result: (f32, f32, (f32, f32), Vec<i32>)) {
    println!("Your MA is: {} ({}:{})", result.0, result.3[0], result.3[1]);
    println!("Your PA is: {} ({}:{})", result.1, result.2 .0, result.2 .1);
    println!(
        "Acc V1: {}% Grade: {}",
        crate::calculate::percent_v1(&result.3),
        crate::calculate::grade(crate::calculate::percent_v1(&result.3))
    );
    println!(
        "Acc V2: {}% Grade: {}",
        crate::calculate::percent_v2(&result.3),
        crate::calculate::grade(crate::calculate::percent_v2(&result.3))
    );
}
