extern crate miniz_oxide;

use miniz_oxide::inflate::decompress_to_vec;
use std::fs::File;
use std::io::Read;

const PATH: &str = "../../Downloads/engi.png";
const IDAT: [u8; 4] = [0x49, 0x44, 0x41, 0x54];
const IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];
const HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

#[derive(Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
struct Output {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

const NILOUT: Output = Output {
    width: 0,
    height: 0,
    pixels: Vec::new(),
};

fn get_output(buf: Vec<u8>) -> Result<Output, String> {
    let mut width: u32;
    let mut height: u32;
    let mut bit_depth: u8;
    let mut color_type: u8;

    let test: bool = check_pattern(&buf, 0xF, IHDR);
    println!("{}", test);

    for (idx, byte) in buf.iter().enumerate() {
        if check_pattern(&buf, idx, IHDR) {
            width = u32::from_be_bytes([
                buf[idx + 1],
                buf[idx + 2],
                buf[idx + 3],
                buf[idx + 4],
            ]);
            height = u32::from_be_bytes([
                buf[idx + 5],
                buf[idx + 6],
                buf[idx + 7],
                buf[idx + 8],
            ]);
            
            bit_depth = buf[idx + 9];
            color_type = buf[idx + 10];

            println!("Width: {}", width);
            println!("Height: {}", height);
            println!("Bit Depth: {}", bit_depth);
            println!("Color Type: {}", color_type);
        }
    }
    
    Ok(NILOUT)
}

fn check_pattern(buffer: &Vec<u8>, idx: usize, pattern: [u8; 4]) -> bool {
    if idx < 4 {
        return false;
    }
    let last_four: [u8; 4] = [buffer[idx-3],buffer[idx-2],buffer[idx-1],buffer[idx]];
    // println!("{:X?}", last_four);

    last_four == pattern
}

// just used to get bytes from image for development
// will use latter functions in actaully program
fn main() {
    let mut f = match File::open(PATH){
        Ok(f) => f,
        _ => panic!("Error reading file"),
    };

    let mut buffer = Vec::new();

    match f.read_to_end(&mut buffer) {
        Ok(_) => (),
        _ => panic!("Error reading file"),
    }

    // println!("Bytes: {:X?}", buffer);
    // we now have the bytes of the image
    // can now pass to new function

    match get_output(buffer) {
        Ok(output) => println!("{:?}", output),
        Err(err) => println!("{}", err),
    }
}
