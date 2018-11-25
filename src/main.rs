extern crate colored;
extern crate rand;

use rand::{thread_rng, Rng, random};
use std::fs;
use std::path::Path;
use std::env;
use colored::*;

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const MAX_HEIGHT: i8 = 15;
const MIN_HEIGHT: i8 = 8;

const GRASS_DEPTH: i8 = 2;
const ROCK_DEPTH: i8 = 8;

fn main() {
    mkdir();
    let dirt1 = "▓▓".red().on_black();
    let dirt2 = "▒▒".red().on_black();
    let grass1 = "██".green();
    let grass2 = "▓▓".green().on_black();
    let rock1 = "▒▒".white().on_black();
    let rock2 = "▓▓".white().on_black();
    let mut rng = thread_rng();
    let mut height: [i8; WIDTH] = [0; WIDTH];
    let mut blocks: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    height[0] = rng.gen_range(MIN_HEIGHT, MAX_HEIGHT);
    for x in 1..WIDTH {
        let mut next = height[x-1];
        for _ in 0..=1 {
            next += rng.gen_range(-2, 2);
        }
        if next > MAX_HEIGHT {
            next = MAX_HEIGHT;
        }
        else if next < MIN_HEIGHT {
            next = MIN_HEIGHT;
        }
        height[x] = next;
    }
    for x in 1..(WIDTH - 1) {
        if random() {
            height[x] = ((height[x] as i16 + height[x-1] as i16 + height[x+1] as i16) / 3) as i8;
        }
    }
    for x in 0..WIDTH {
        for y in 0..=height[x] {
            let depth = height[x] - y;
            let indicator = rng.gen_range(0, depth + 1) + depth * 2 / 3;
            if indicator < GRASS_DEPTH {
                blocks[y as usize][x] = 2;
            }
            else if indicator > ROCK_DEPTH {
                blocks[y as usize][x] = 3;
            }
            else {
                blocks[y as usize][x] = 1;
            }
        }
    }

    println!("");
    for y in 1..=HEIGHT {
        print!("  ");
        for x in 0..WIDTH {
            match blocks[HEIGHT - y][x] {
                1 => {
                    if random() || random() {
                        print!("{}", dirt1);
                    }
                    else {
                        print!("{}", dirt2);
                    }
                },
                2 => {
                    if random() {
                        print!("{}", grass1);
                    }
                    else {
                        print!("{}", grass2);
                    }
                },
                3 => {
                    if random() {
                        print!("{}", rock1);
                    }
                    else {
                        print!("{}", rock2);
                    }
                },
                _ => print!("  ")
            }
        }
        println!("");
    }
    println!("");
}

pub fn mkdir() {
    let mut args = env::args().into_iter();
    let len = args.len();
    args.next();
    if len > 1 {
        match env::current_dir() {
            Ok(cwd_p) => {
                let cwd = Path::new(&cwd_p);
                for arg in args {
                    let path = cwd.clone().join(arg);
                    fs::DirBuilder::new().recursive(true).create(path).unwrap();
                }
            },
            Err(error) => {
                panic!("Couldn't read current directory: {}", error);
            }
        }
    }
}
