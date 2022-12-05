use std::error::Error;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn main() {
    print!("Please pick a number: ");
    let _ = io::stdout().flush();
    let mut user_input = String::new();
    let stdin = io::stdin();
    let ret_input = stdin.lock().read_line(&mut user_input);
    println!(""); //newline after read
  
    let Ok(i) = ret_input else {
        return //exits application
    };
    // can use i as an int freely now!
    let print_string = handle_user_input(i, user_input.clone());
    println!("{print_string}");

    let user_number_result = user_input.parse::<i32>();
//    let print_value = match ret_input {
//        Ok(i) => handle_user_input(i, user_input),
//        Err(e) => format!("{e}"),
//    };
//    println!("{print_value}")

//    match ret_input {
//        Ok(i) => {
//            let print_string = handle_user_input(i, user_input);
//            println!("{print_string}")
//        },
//        Err(e) => println!("{e}")
//    }

//    if let Ok(i) = ret_input {
//        let print_string = handle_user_input(i, user_input);
//        println!("{print_string}")
//    }

}

fn might_fail(i: i32) -> Result<i32,String> {
    if i < 10 {
        return Err("failed".to_string());
    }

    return Ok(i);
}

fn might_return(i: i32) -> Option<i32> {
    if i < 10 {
        return None;
    }

    return Some(i);
}

fn might_panic(i: i32) -> i32 {
    if i < 10 {
        panic!("Oh no! you picked less then 10!!!")
    }

    return i;
}

pub fn add(int1: i32, int2: i32) -> i32 {
    return int1 + int2;
}

fn add2(int1: i32, int2: i32) -> i32 {
    int1 + int2
}


fn handle_user_input(i: usize, input: String) -> String {
    format!("You input {i} number of characters: {input}")
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

    #[test]
    fn test_handle_user_input() {
        let str1 = "You input 2 number of characters: 1".to_string();
        let str2 = handle_user_input(2, "1".to_string());
        assert_eq!(str1,str2);
    }
}