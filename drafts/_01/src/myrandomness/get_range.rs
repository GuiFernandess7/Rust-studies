use rand::Rng;

pub fn get_rand_nums(initial: i32, end: i32) -> i32 {
    let random_num = rand::thread_rng().gen_range(initial..end);
    return random_num
}