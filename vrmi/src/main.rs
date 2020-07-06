use std::io;
use std::io::prelude::*;
use std::fs::File;

use nom::bytes::complete::tag;
use nom::number::complete::*;
use nom::{error::ErrorKind, Err};

struct GLTFHeader<'a> {
    magic: &'a [u8],
    version: u32,
    length: u32,
}

fn parse_header(input : &[u8]) -> Result<GLTFHeader, Err<(&[u8], ErrorKind)>> {
    let (input, magic) = tag(b"\x67\x6c\x54\x46")(input)?;
    let (input, version) = le_u32(input)?;
    let (_input, length) = le_u32(input)?;

    Ok(GLTFHeader{magic, version, length})
}

fn main() -> io::Result<()> {
    let path = "test.vrm";
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    f.read_to_end(&mut buffer)?;

    println!("filename: {}", path);

    match parse_header(&buffer) {
        Ok(header) => {
            println!("header:");
            println!("  magic: {}", String::from_utf8(header.magic.to_vec()).unwrap());
            println!("  version: {}", header.version);
            println!("  length: {}", header.length);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
    Ok(())
}
