fn binary_search(nums: Vec<i32>, number: i32) -> Option<usize>{
    let mut low = 0;
    let mut high = nums.len();

    while low < high {
        let mut middle = low + (high - low) / 2;

        if nums[middle] < number {
            low = middle + 1
        }
        else if nums[middle] > number {
            high = middle
        }
        else {
           return Some(middle);
        }
    }
    None
}

fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];

    let mut i = 0;
    let mut j = 0;

    while j < len - 1{
        if slice[j] < pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    slice.swap(i, len - 1);
    return i;
}

fn quick_sort(){}


fn main() {
    let arr = Vec::from([1, 2, 3, 4, 5]);
    let num_to_find: i32 = 2;

    let result = binary_search(arr, num_to_find);
    match result {
        Some(index) => println!("Found {} at position {}!", num_to_find, index),
        None => println!("Number not found"),
    }
}
