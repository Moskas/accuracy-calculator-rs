use std::fs::File;
use std::io::prelude::*;

pub fn write() -> std::io::Result<()>{
    let mut file = File::create("result.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

#[test]
fn write_test(){
    assert_eq!(write(),Some(Ok));
}