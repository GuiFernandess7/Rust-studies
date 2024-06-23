#![allow(unused)]

use std::char::ToLowercase;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

mod fibonacci;

fn main() {
    let arr_2 = [1,2,3,4,5,6,7,8,9,10,15];
    let mut loop_idx = 0;

    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
}
