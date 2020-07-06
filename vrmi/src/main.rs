use std::io;
use std::io::prelude::*;
use std::fs::File;

use nom::bytes::complete::{tag, take};
use nom::number::complete::*;
use nom::{error::ErrorKind, Err};

struct GLTFHeader<'a> {
    magic: &'a [u8],
    version: u32,
    length: u32,
}

struct GLTFChank<'a> {
    chank_length: u32,
    chank_type: &'a [u8],
    chank_data: &'a [u8],
}

fn parse_header(input : &[u8]) -> Result<(&[u8], GLTFHeader), Err<(&[u8], ErrorKind)>> {
    let (input, magic) = tag(b"\x67\x6c\x54\x46")(input)?;
    let (input, version) = le_u32(input)?;
    let (input, length) = le_u32(input)?;

    Ok((input, GLTFHeader{magic, version, length}))
}

fn parse_chank(input: &[u8]) -> Result<GLTFChank, Err<(&[u8], ErrorKind)>> {
    let (input, chank_length) = le_u32(input)?;
    let (input, chank_type) = take(4u8)(input)?;
    let (_input, chank_data) = take(chank_length)(input)?;

    Ok(GLTFChank{chank_length, chank_type, chank_data})
}

fn main() -> io::Result<()> {
    let path = "test.vrm";
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    f.read_to_end(&mut buffer)?;

    println!("filename: {}", path);

    match parse_header(&buffer) {
        Ok((input, header)) => {
            println!("header:");
            println!("  magic: {}", String::from_utf8(header.magic.to_vec()).unwrap());
            println!("  version: {}", header.version);
            println!("  length: {}", header.length);

            match parse_chank(&input) {
                Ok(chank) => {
                    println!("chank{}:", 0);
                    println!("  length: {}", chank.chank_length);
                    println!("  type: {}", String::from_utf8(chank.chank_type.to_vec()).unwrap());
                    println!("  data: {}", String::from_utf8(chank.chank_data.to_vec()).unwrap());
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
    Ok(())
}
