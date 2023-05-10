use std::{io::{self, Write}, num::ParseIntError, error::Error, fmt, collections::HashMap};

struct Point2D {
    x: i32,
    y: i32,
}

struct Point(i32,i32);

impl Point {

}

struct Marek;

type SpHashMap = HashMap<String,String>;

fn main() -> Result<(),Box<dyn Error>> {

    let my_point = (20,20);

    let p = Point(20,20);
    let marek = Person{
        age: 34,
        name: "Marek".to_string()
    };

    let mut jon = Person::new(51,"jon".to_string());

    let jons_age = jon.get_age();

    jon.birthday();

    Ok(())
}

struct Person {
    age: i32,
    name: String,
}

impl Person {
    fn new(age: i32, name: String) -> Self {
        Person { age: age, name: name }
    }

    fn get_age(&self) -> i32 {
        self.age
    }

    fn birthday(&mut self) {
        self.age = self.age + 1
    }
}




#[cfg(test)]
mod tests {
   

}