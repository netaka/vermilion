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

fn parse_chank(input: &[u8]) -> Result<(&[u8], GLTFChank), Err<(&[u8], ErrorKind)>> {
    let (input, chank_length) = le_u32(input)?;
    let (input, chank_type) = take(4u8)(input)?;
    let (input, chank_data) = take(chank_length)(input)?;

    Ok((input, GLTFChank{chank_length, chank_type: chank_type.to_vec(), chank_data: chank_data.to_vec()}))
}

fn parse_gltf(input: &[u8]) -> Result<GLTFContainer, Err<(&[u8], ErrorKind)>> {
    let mut gltf: GLTFContainer = Default::default();

    let (input, header) = parse_header(&input)?;
    gltf.header = header;

    let mut chank_input = input;
    while chank_input.len() > 0 {
        let (input, chank) = parse_chank(&chank_input)?;
        gltf.chank.push(chank);
        chank_input = input;
    }
    
    Ok(gltf)
}

impl GLTFHeader {
    fn print(&self) {
        println!("  magic: {}", String::from_utf8(self.magic.to_vec()).unwrap());
        println!("  version: {}", self.version);
        println!("  length: {}", self.length);
    }
}

impl GLTFChank {
    fn print(&self) {
        println!("  length: {}", self.chank_length);
        let chank_type = String::from_utf8(self.chank_type.to_vec()).unwrap();
        println!("  type: {}", chank_type);
        match chank_type.as_str() {
            "JSON" => print_json(String::from_utf8(self.chank_data.to_vec()).unwrap()),
            "BIN\x00" => println!("  data: <binary data>"),
            _ => println!("  data: <unknown type>"),
        } 
    }
}

impl GLTFContainer {
    fn print(&self) {
        println!("header:");
        self.header.print();
        let mut count = 0;
        println!("len {}", self.chank.len());
        for chank in &self.chank {
            println!("chank{}:", count);
            chank.print();
            count += 1;
        }
    }
}

fn print_json(json: String) {
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    let json = serde_json::to_string_pretty(&v).unwrap();
    println!("{}", json);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: vrmi <filename>");
    }
    let path = &args[1];
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    f.read_to_end(&mut buffer)?;

    println!("filename: {}", path);

    match parse_gltf(&buffer) {
        Ok(gltf) => {
            gltf.print();
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
    Ok(())
}
