#![allow(dead_code, unused)]

use clap::Parser;
use std::io::{Error, ErrorKind};

mod ch5_1;
mod ch5_2;
mod ch5_3;
mod ch5_4;
mod ch5_5;
mod ch5_6;
mod ch5_7;
mod ch5_8;
mod ch5_cpu1;
mod ch5_cpu2;
mod ch5_cpu3;
mod ch5_cpu4;
mod ch5_q;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ch: u8,
}

fn main() {
    // let args = Args::parse();

    // println!("-----------------------------------------------------------");
    // println!("chapter: {}\n", args.ch);
    match 8 {
        1 => ch5_1::main(),
        2 => ch5_2::main(),
        3 => ch5_3::main(),
        4 => ch5_4::main(),
        5 => ch5_5::main(),
        6 => ch5_6::main(),
        7 => ch5_7::main(),
        8 => ch5_8::main(),
        9 => ch5_cpu1::main(),
        10 => ch5_cpu2::main(),
        11 => ch5_cpu3::main(),
        12 => ch5_cpu4::main(),
        _ => println!("Invalid chapter num"),
    }
}
