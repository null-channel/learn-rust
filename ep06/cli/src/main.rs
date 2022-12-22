#![allow(dead_code)]
use self::Mammal::*;

fn main() {

   let marek = Person{
     first_name: String::from("Marek"),
     last_name: String::from("Counts"),
     age: 35,
     is_admin: true,
   };
   let thing = Mammal::Human(marek);
}

struct Person {
    first_name: String,
    last_name: String,
    age: i8,
    is_admin: bool,
}

enum Mammal {
    Human(Person),
    Dog(i8,bool),
    Cat {lives:i8,is_hungry: bool},
    Platypus(),
    Rabbit{age: i8},
}

fn age(mammal: Mammal) -> i8 {
    match mammal {
        Human(human) => human.age,
        Dog(age,_happy ) => age,
        Cat { lives, is_hungry: _ } => lives -1,
        Platypus() => 1,
        Rabbit{age} => age,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
    }
}