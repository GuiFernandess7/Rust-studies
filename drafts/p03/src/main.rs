use rand::Rng;
use std::thread;
use std::time::Duration;

// SEARCH O(N^2) (?)

fn get_random_number(initial: i32, end: i32) -> i32 {
    let random_number = rand::thread_rng().gen_range(initial..end);
    return random_number
}

fn main() {
    let last_value: i32 = 100;
    let mut nums: Vec<i32> = Vec::new();
    let mut value_found: bool = false;

    for n in 0..last_value {
        nums.push(n);
    }

    loop {
        let mut new_rand_n = get_random_number(0, 400);
        println!("Searching for value {} ...", new_rand_n);
        for n in nums.iter(){
            if *n == new_rand_n {
                value_found = true;
                println!("Value {} Found!", n);
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }

        if value_found == true {
            break;
        }

        println!("Value {} not found.", new_rand_n);
    }

    if !value_found {
        println!("Sorry, value was not found in vector of length {}", last_value);
    }
}
