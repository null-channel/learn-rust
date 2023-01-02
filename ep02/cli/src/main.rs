
fn main() {
    let n1 = 5;
    println!("number 1 is: {n1}");
    let n2 = add5(n1);
    println!("number 2 is: {n2}");
    let n3 = add(n1, n2);
    println!("number 3 is: {n3}");

    /*
     * n3 is not mutable!!!
    n3 = 6;
    println!("number 3 is now: {n3}");
    */

    let mut n4 = add(n3,n2);
    println!("number 4 is: {n4}");
    n4 = 5;
    println!("number 4 is now: {n4}");

    
}

fn add(int1: i32, int2: i32) -> i32 {
    int1 + int2
}

pub fn add5(mut int1: i32) -> i32 {
    int1 = int1 + 5;
    return int1;
}
