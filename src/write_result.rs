use std::fs::File;
use std::io::prelude::*;

pub fn write(result: (f32, f32, (f32, f32), f32, f32, &Vec<i32>)) -> std::io::Result<()> {
    let mut file = File::create("result.csv")?;
    //write!(&mut file, "formatted {:?}", result)?;
    write!(
        &mut file,
        "{}",
        // formatting MA,PA,ACCV1,GRADE,ACCV2,GRADE,JUDGEMENTS
        /*
        format!(
            "Your MA is: {} ({}:{}) \nYour PA is: {} ({}:{})\nAcc V1: {}% Grade: {}\nAcc V2: {}% Grade: {}",
            result.0, result.5[0], result.5[1], result.1, result.2 .0, result.2 .1,result.3,crate::calculate::grade(result.3),result.4,crate::calculate::grade(result.4)
        )*/
        format!(
            "{},{},{},{},{},{},{:?}",
            result.0,
            result.1,
            result.3,
            crate::calculate::grade(result.3),
            result.4,
            crate::calculate::grade(result.4),
            result.5
        )
    )?;
    Ok(())
}
