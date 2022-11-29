use std::io::{self, BufRead, Write};

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
    // can use i freely now!
    let print_string = handle_user_input(i, user_input);
    println!("{print_string}")

    if i > 4 {
        println!("i is larger then 4");
    }

    if i == 4 {
        println!("i is 4!");
    } else if i > 5 {
        println!("i is large!");
    } else {
        print!("i is small!")
    }
    
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