
fn main() {
    let number = 5;
    let number2 = add(number,number);
    let number3 = add2(number,number);
    let number4 = {
        number + number
    };
    let number5 = match number {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        _ => 0,
    };
    let number6 = if number == 3 {
        5
    } else {
        0
    };

    println!("n1: {}", number);
    println!("n2: {}", number2);
    println!("n3: {}", number3);
    println!("n4: {}", number4);
    println!("n5: {}", number5);
    println!("n6: {}", number6);
}

fn add(int1: i32, int2: i32) -> i32 {
    int1 + int2
}

fn add2(int1: i32, int2: i32) -> i32 {
    return int1 + int2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let number = 5;
        let number2 = add(number,number);
        assert_eq!(number2, number + number);
        assert_ne!(number2, number - number);
    }
}