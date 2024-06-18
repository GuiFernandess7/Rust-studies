use rand::Rng; // 0.6.5
use std::io;

fn binary_search(arr: &Vec<i32>, target: i32) -> bool{
    let mut high = arr.len();
    let mut low = 0;

    while low < high {
        let mut middle = low + (high - low) / 2;
        let curr_value = arr[middle];

        if target < curr_value {
            high = middle // inclusivo
        }
        else if target > curr_value {
            low = middle + 1 // exclusivo
        }
        else {
            return true;
        }
    }
    return false;
}

fn check_if_sorted(nums: &Vec<i32>) -> bool{
    for n in 0..nums.len() - 1 {
        if nums[n] > nums[n + 1] {
            return false;
        }
    }
    true
}

fn quick_sort(slice: &mut [i32]){
    if !slice.is_empty(){
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
    }
}

fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let mut i = 0;
    let mut j = 0;
    let pivot = slice[len - 1];

    while j < len {
        if slice[j] < pivot{
            slice.swap(i, j);
            i += 1
        }
        j += 1
    }
    slice.swap(i, len - 1);
    return i;

}

fn main(){
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..100).map(|_| rng.gen_range(0, 1000)).collect();
    let is_sorted = check_if_sorted(&arr);

    if !is_sorted{
        quick_sort(&mut arr)
    }

    loop {
        println!("Type a number to search or type 'q' to exit or 's' to see the array");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the line");

        if input.trim() == "q" {
            println!("Exiting");
            break;
        }

        else if input.trim() == "s" {
            println!("{:?}", &arr);
            continue;
        }

        else {
            let target: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid Entry.");
                    continue;
                }
            };

            let result = binary_search(&arr, target);
            if result {
                println!("Found!");
            } else {
                println!("Number {} Not Found", target);
            }
        }
    }
}
