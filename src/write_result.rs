use std::fs::File;
use std::io::prelude::*;

pub fn write(result: (f32, f32, (f32, f32), &Vec<i32>)) -> std::io::Result<()> {
    let mut file = File::create("result.txt")?;
    write!(&mut file, "formatted {:?}", result)?;
    Ok(())
}
