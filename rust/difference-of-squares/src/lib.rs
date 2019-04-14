pub fn square_of_sum(n: u32) -> u32 {
    let mut x = 0; 
    for i in 1..(n+1) {
        x += i;
    }
    return x.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut x = 0;
    for i in 1..(n+1) {
        x += i.pow(2);
    }
    return x;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum (n) - sum_of_squares(n);
}