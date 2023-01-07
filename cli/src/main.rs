fn main() {
    let number = 5;
    let number2 = add(number,number);
    println!("The number is {number2}");
}

// 
pub fn add(int1: i32, int2: i32) -> i32 {
    return int1 + int2;
}

// The idiomatic rust way of doing it.
fn add2(int1: i32, int2: i32) -> i32 {
    int1 + int2
}

