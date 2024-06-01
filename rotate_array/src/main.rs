fn main() {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let mut k: i32 = 3;

    let mut l = 0;
    let mut r = arr.len() - 1;

    while l < r {
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }

    let mut l = 0;
    let mut r = (k - 1) as usize;

    while l < r {
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }

    let mut l = k as usize;
    let mut r = (arr.len() - 1) as usize;

    while l < r {
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }

    println!("{:?}", arr);
}
