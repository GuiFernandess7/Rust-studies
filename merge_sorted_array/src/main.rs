use clap::{command, Arg};

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> (){
        let i: i32 = m + n;
        let mut j: i32 = 0;

        for idx in 0..i {
            println!("{}", nums1[idx as usize]);
        }
    }
}

fn main() {
    let matches = command!().arg(Arg::new("list")).get_matches();
    let list_match = matches.get_one::<String>("list").unwrap().split(",");
    let mut myList: Vec<i32> = list_match.map(|s| s.parse::<i32>().expect("Unable to parse list itens to integer")).collect();
    let mut myList2: Vec<i32> = vec![3, 4, 5, 6];

    Solution::merge(&mut myList, 2, &mut myList2, 5);

}
