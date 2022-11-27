use std::io::{self, BufRead, Write};

fn main() {
    print!("Please pick a number: ");
    let _ = io::stdout().flush();
    let mut user_input = String::new();
    let stdin = io::stdin();
    let ret_input = stdin.lock().read_line(&mut user_input);
    if let Ok(i) = ret_input {
        println!(""); // newline
        println!("You input {i} number of characters: {user_input}")
    }
}

pub fn add(int1: i32, int2: i32) -> i32 {
    return int1 + int2;
}

fn add2(int1: i32, int2: i32) -> i32 {
    int1 + int2
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