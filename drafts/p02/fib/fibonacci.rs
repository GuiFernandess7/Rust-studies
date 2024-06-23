pub fn sum_of_terms(value: i32) -> i32{
    let mut sum = 0;

    if (value <= 2){
        value
    }

    for i in 2..value{
        sum += (i - 1) + (i + 1)
    }

    return sum;
}
