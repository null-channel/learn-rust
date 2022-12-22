
fn main() {
    let n1 = 5;
    let n2 = add5(n1);
    println!("{n2}");
    let mut s1 = String::from("Ive added ");
    
    add_five_to_string( &mut s1);
    println!("add string: {s1}");
    let mut thing = Thing { num: 10 };

    add_five_to_thing(&mut thing);
    let num = thing.num;
    println!("Your number is: {num}")
}

pub fn add5(mut int1: i32) -> i32 {
    int1 = int1 + 5;
    return int1;
}

fn add_five_to_string(string1: &mut String) {
    string1.push_str("Five")
}

struct Thing {
    num: i32
}

fn add_five_to_thing(thing: &mut Thing) {
    thing.num = thing.num + 5;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut number = 5;
        let number2 = add5(number);
        assert_eq!(number2, number + number);
        assert_ne!(number2, number - number);
    }
}