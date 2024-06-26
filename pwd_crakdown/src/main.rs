use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid amount of arguments");
        println!("Example: cargo run <sha256 hash>");
        exit(1);
    }

    let wanted_hash = &args[1];
    let password_file = "./pwdlist.txt";
    let mut attempts = 1;
    let mut found: bool = false;

    println!("Attempting to crack: {}!\n", wanted_hash);

    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));

        //println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}",
                                            attempts,
                                            std::str::from_utf8(&password).unwrap(),
                                            password_hash
            );
            found = true;
        }
        attempts += 1;
    }

    if !found {
        println!("Password hash not found")
    }
}
