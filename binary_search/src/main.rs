use clap::{command, Arg};

fn binary_search(nums: &Vec<i32>, number: i32) -> Option<i32>{
    // Find number in list
    let mut low: i32 = 0;
    let mut high: i32 = nums.len().try_into().unwrap();

    while low < high{
        let middle: i32 = low + (high - low) / 2;
        let value: i32  = nums[middle as usize];

        if value == number {
            return Some(number);
        }
        else if value > number {
            high = middle;
        }
        else {
            low = middle + 1;
        }
    }
    None
}

fn check_if_sorted(nums: &Vec<i32>) -> bool{
    for n in 0..nums.len() - 1 {
        if nums[n] > nums[n + 1] {
            return false;
        }
    }
    true
}

fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    slice.swap(i, len - 1);
    return i;
}

fn quick_sort(slice: &mut [i32]){
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len])
    }
}

fn main() {
    let matches = command!().arg(Arg::new("value")).arg(Arg::new("list")).get_matches();

    let value_match = matches.get_one::<String>("value").unwrap();
    let list_match = matches.get_one::<String>("list").unwrap().split(",");

    let target_value: i32 = value_match.parse::<i32>().expect("Unable to parse value as integer");
    let mut mylist: Vec<i32> = list_match.map(|s| s.parse::<i32>().expect("Unable to parse list item as integer")).collect();

    let index  = binary_search(&mylist, target_value);
    let is_sorted: bool = check_if_sorted(&mylist);

    if !is_sorted {
        let mut slice_list = mylist.as_mut_slice();
        quick_sort(&mut slice_list);
    }

     match index {
        Some(index) => println!("Number found!: {}", index),
        None => println!("Number not found"),
    }

}
