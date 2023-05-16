use std::{error::Error, fmt};

enum Messages {
    Hello,
    Quit,
    ChangeColor(i32,i32,i32),
    Move{x:i32, y:i32},
    Write(String),
    ChangePosition(Point),
}

impl Messages {
    fn print_data(&self) {
        match self {
            Messages::Hello => println!("Hello"),
            Messages::Quit => println!("Quit"),
            Messages::ChangeColor(r,g,b) => {
                println!("ChangeColor({}, {}, {})", r, g, b)
            },
            Messages::Move {x, y} => println!("Move to ({}, {})", x, y),
            Messages::Write(s) => println!("Write {}", s),
            Messages::ChangePosition(Point {x, y}) => println!("ChangePosition to ({}, {})", x, y),
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Messages::Hello => write!(f, "Hello"),
            Messages::Quit => write!(f, "Quit"),
            Messages::ChangeColor(r,g,b) => write!(f, "ChangeColor({}, {}, {})", r, g, b),
            Messages::Move{x, y} => write!(f, "Move to ({}, {})", x, y),
            Messages::Write(s) => write!(f, "Write {}", s),
            Messages::ChangePosition(Point {x, y}) => write!(f, "ChangePosition to ({}, {})", x, y),
        }
    }
}

fn main() -> Result<(),Box<dyn Error>> {

  let hello_message =  Messages::Hello;
  hello_message.print_data();
  println!("{}", hello_message);

  let point = Point {x: 1, y: 30};
  let change_position_message = Messages::ChangePosition(point);
  change_position_message.print_data();

  let change_color_message = Messages::ChangeColor(255, 255, 255);
  change_color_message.print_data();

  let move_message = Messages::Move {x: 10, y: 20};
  move_message.print_data();

  None::<i32>;

  Ok(())
}


#[cfg(test)]
mod tests {
   

}