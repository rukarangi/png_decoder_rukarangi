extern crate miniz_oxide;

use miniz_oxide::inflate::decompress_to_vec;
use std::fs::File;
use std::io::Read;

const PATH: &str = "../../Programming/ascii-image-generator/rust-module/data/chrome-512.png";
const IDAT: [u8; 4] = [0x49, 0x44, 0x41, 0x54];
const IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];
const HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
const COLOR_TYPE_TO_PIXEL_DIST: [u8; 7] = [1, 0, 3, 1, 4, 0, 4];

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

fn get_output(buf: Vec<u8>) -> Result<Output, &'static str> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: u8 = 0;
    let mut color_type: u8 = 0;

    let mut dat_appended: Vec<u8> = Vec::new();

    let test: bool = check_pattern(&buf, 0xF, IHDR);
    println!("{}", test);

    for (idx, byte) in buf.iter().enumerate() {
        // find header chunk and get data out
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

        if check_pattern(&buf, idx, IDAT) {
            println!("{:X?}", [
                buf[idx - 7],
                buf[idx - 6],
                buf[idx - 5],
                buf[idx - 4],
            ]);
            let length: u32 = u32::from_be_bytes([
                buf[idx - 7],
                buf[idx - 6],
                buf[idx - 5],
                buf[idx - 4],
            ]);
            println!("{}", length);
            let dat = &buf[(idx+1)..(idx + 1 + length as usize)];
            //println!("{:X?}", dat);
            dat_appended.extend_from_slice(dat);
            //println!("{:X?}", dat_appended);
        }
    }
    
    // attempt to decompress, [2..] wierdness with zlib
    let dat_inflated: Vec<u8> = match decompress_to_vec(&dat_appended[2..]) {
        Ok(dat) => dat,
        Err(err) => panic!("Failed to decompress: {:?}", err),
    };
    
    let dimension2_shiz = become_2d(&dat_inflated, width as usize, height as usize);

    let pixels: Vec<Pixel> = pixel_maker(dat_inflated, width, height, bit_depth, color_type);

    Ok(NILOUT)
}

fn become_2d(data: &Vec<u8>, width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            
        }
    }

    return output;
}

fn pixel_maker(data: Vec<u8>, width: u32, height: u32, bit_depth: u8, color_type: u8) -> Vec<Pixel> {
    println!("{:X?}", &data[..1000]);
    let mut test = Vec::new();
    let filter: u8 = data[0];
    println!("{:X?}", filter);
    for (idx, byte) in data.iter().enumerate() {
        if idx == 0 || idx & (width + 1) as usize == 0 {
            //test.push(idx as u8);
            continue;
        }
        test.push(*byte);

        let good_byte = match apply_filter(filter, &data, 32, idx, bit_depth, color_type) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };
    }
    
    println!("{:X?}", &test[..1000]);

    return Vec::new();
} 

fn apply_filter(filter: u8, data: &Vec<u8>, length: usize, target: usize, bit_depth: u8, color_type: u8) -> Result<u8, &'static str> {
    if (8 * COLOR_TYPE_TO_PIXEL_DIST[color_type as usize]) as usize != length {
        return Err("Tried to apply filter to wrong lengthed section");
    }

    // let target_bytes: Vec<u8> = vec![data[target]]

    match filter {
        0 => return Ok(data[target]),
        _ => return Err("This filter type is not implemented"),
    }

    Ok(0x0)
}

fn check_pattern(buffer: &Vec<u8>, idx: usize, pattern: [u8; 4]) -> bool {
    if idx < 4 {
        return false;
    }
    let last_four: [u8; 4] = [buffer[idx-3],buffer[idx-2],buffer[idx-1],buffer[idx]];
    // println!("{:X?}", last_four);

    return last_four == pattern;
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
