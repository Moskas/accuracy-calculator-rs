//  Printing out results
pub fn print_out(result: (f32, f32, (f32, f32), f32, f32, &Vec<i32>)) {
    println!("Your MA is: {} ({}:{})", result.0, result.5[0], result.5[1]);
    println!("Your PA is: {} ({}:{})", result.1, result.2 .0, result.2 .1);
    println!(
        "Acc V1: {}% Grade: {}",
        result.3,
        crate::calculate::grade(result.3)
    );
    println!(
        "Acc V2: {}% Grade: {}",
        result.4,
        crate::calculate::grade(result.4)
    );
}
