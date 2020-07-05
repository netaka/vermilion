use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let path = "test.vrm";
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    f.read_to_end(&mut buffer)?;

    println!("filename: {}", path);
    println!("size: {}", buffer.len());

    Ok(())
}
