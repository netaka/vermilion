use std::io;
use std::io::prelude::*;
use std::fs::File;

use nom::bytes::complete::{tag, take};
use nom::number::complete::*;
use nom::{error::ErrorKind, Err};

#[derive(Default)]
struct GLTFHeader {
    magic: Vec<u8>,
    version: u32,
    length: u32,
}

#[derive(Default)]
struct GLTFChank {
    chank_length: u32,
    chank_type: Vec<u8>,
    chank_data: Vec<u8>,
}

#[derive(Default)]
struct GLTFContainer {
    header: GLTFHeader,
    chank: Vec<GLTFChank>,
}

fn parse_header(input : &[u8]) -> Result<(&[u8], GLTFHeader), Err<(&[u8], ErrorKind)>> {
    let (input, magic) = tag(b"\x67\x6c\x54\x46")(input)?;
    let (input, version) = le_u32(input)?;
    let (input, length) = le_u32(input)?;

    Ok((input, GLTFHeader{magic: magic.to_vec(), version, length}))
}

fn parse_chank(input: &[u8]) -> Result<GLTFChank, Err<(&[u8], ErrorKind)>> {
    let (input, chank_length) = le_u32(input)?;
    let (input, chank_type) = take(4u8)(input)?;
    let (_input, chank_data) = take(chank_length)(input)?;

    Ok(GLTFChank{chank_length, chank_type: chank_type.to_vec(), chank_data: chank_data.to_vec()})
}

fn parse_gltf(input: &[u8]) -> Result<GLTFContainer, Err<(&[u8], ErrorKind)>> {
    let mut gltf: GLTFContainer = Default::default();

    let (input, header) = parse_header(&input)?;
    let chank = parse_chank(&input)?;

    gltf.header = header;
    gltf.chank.push(chank);
    
    Ok(gltf)
}

fn print_header(header: &GLTFHeader) -> () {
    println!("header:");
    println!("  magic: {}", String::from_utf8(header.magic.to_vec()).unwrap());
    println!("  version: {}", header.version);
    println!("  length: {}", header.length);
}

fn print_chank(chank: &GLTFChank) -> () {
    println!("chank{}:", 0);
    println!("  length: {}", chank.chank_length);
    println!("  type: {}", String::from_utf8(chank.chank_type.to_vec()).unwrap());
    println!("  data: {}", String::from_utf8(chank.chank_data.to_vec()).unwrap());
}

fn print_gltf(gltf: &GLTFContainer) -> () {
    print_header(&gltf.header);
    print_chank(&gltf.chank[0]);
}

fn main() -> io::Result<()> {
    let path = "test.vrm";
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    f.read_to_end(&mut buffer)?;

    println!("filename: {}", path);

    match parse_gltf(&buffer) {
        Ok(gltf) => {
            print_gltf(&gltf);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
    Ok(())
}
