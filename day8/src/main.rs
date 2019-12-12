#[macro_use]
extern crate log;
extern crate env_logger;
extern crate itertools;
extern crate png;

use itertools::Itertools;
use png::{BitDepth, ColorType, Encoder, *};
use std::env;
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::mem::replace;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
static COLORS: [[u8; 4]; 3] = [[0, 0, 0, 255], [255, 255, 255, 255], [0, 0, 0, 0]];

macro_rules! parse {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().expect("parse failed")
    };
}

fn row(idx: usize) -> usize {
    idx / WIDTH
}
fn col(idx: usize) -> usize {
    idx - row(idx) * WIDTH
}

fn idx(row: usize, col: usize) -> usize {
    row * WIDTH + col
}

fn write_layer(name: &str, layer: Vec<u8>) {
    let file = File::create(format!("{}.png", name)).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set(ColorType::RGBA).set(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&layer).unwrap();
}

fn layer_to_rgba(layer: Vec<u8>) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(layer.len() * 4);
    for color in layer {
        data.extend(COLORS[color as usize].iter())
    }
    data
}

fn main() {
    env_logger::init();
    debug!("start");
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let input = read_to_string(input_file).unwrap();
    let layers: Vec<String> = input
        .chars()
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect();

    let mut pixels: Vec<u8> = vec![2; WIDTH * HEIGHT];
    for (idl, layer) in layers.iter().enumerate() {
        let zeroes = layer.matches("0").count();
        let ones = layer.matches("1").count();
        let twos = layer.matches("2").count();
        println!("{}: {} {} {} -> {}", idl, zeroes, ones, twos, ones * twos);
        debug!("{:?}", pixels);
        for (idp, pixel) in layer.chars().enumerate() {
            match pixels[idp] {
                2 => {
                    let p = pixel.to_digit(10).unwrap();
                    replace(&mut pixels[idp], p as u8);
                }
                _ => {}
            }
        }
    }

    write_layer("output", layer_to_rgba(pixels));
}
