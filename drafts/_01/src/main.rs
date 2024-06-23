#![allow(unused)]

use std::char::ToLowercase;
use std::io;
use myrandomness::get_range::get_rand_nums;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

mod casting;
mod myrandomness;

fn input() -> String{
    let mut value = String::new();
    io::stdin().read_line(&mut value)
        .expect("Didn't receive an input");
    value.trim().to_string()
}

fn validate_age(age: i32) -> bool {
    age >= 1 && age <= 115
}

fn main() {
    println!("What is your name?    ");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    let name = input();

    if name.trim() == String::from("Guilherme"){
        println!("Hello Again!")
    } else {
        println!("{}", greeting);
    }

    println!("How old are you?    ");
    let age = input().parse::<i32>();

    match age {
        Ok(age_n) if age_n >= 1 && age_n <= 115 => {
            println!("Nice! Got it. {} yo", age_n);
        },
        Ok(_) => {
            println!("Not a valid age!");
        },
        Err(e) => println!("Not a valid digit: {}", e),
    }
}

