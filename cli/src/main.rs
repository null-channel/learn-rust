use std::{error::Error, fmt};

trait Area {
    fn area(&self) -> i32;
}

impl Area for String {
    fn area(&self) -> i32 {
        42
    }
}

trait Height {
    fn get_height(&self) -> i32;
}

trait Width {
    fn get_width(&self) -> i32;
}

struct Square {
    width: i32,
}

struct Rectangle {
    width: i32,
    height: i32,
}

enum Polygons {
    Square(i32),
    Rectangle(i32,i32),
    EqTriangle{h: i32,w:i32}
}

impl Area for Polygons {
    fn area(&self) -> i32 {
        match self {
            Polygons::Square(side_length) => {
                *side_length * *side_length
            },
            Polygons::Rectangle(length,width) => {
                *length * *width
            },
            Polygons::EqTriangle{h,w} => {
                (*h * *w) / 2
            }
        }
    }
}

impl Area for Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

impl Area for Square {
    fn area(&self) -> i32 {
        self.width * self.width
    }
}

fn get_area<T>(x: T) -> i32 
where T: Area {
    x.area()
}

#[derive(Debug,PartialEq)]
struct OldPoint {
    x: i32,
    y: i32,
}
// Implementing the Display trait for OldPoint
impl fmt::Display for OldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }    
}

struct Point<T> {
    x: T,
    y: T,
}

/*
impl fmt::Display for Point<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }    
}
*/

/*
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }    
}
*/

impl <T> fmt::Display for Point<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }    
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Self{x,y}
    }

    fn update(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

fn main() -> Result<(),Box<dyn Error>> {

    let poly = Polygons::EqTriangle { h: 5, w: 5 };

    let rec = Rectangle{width: 5, height: 5};
    
    let areaRec = get_area(rec);

    let areaPoly = get_area(poly);

    Ok(())
}

fn print_point<T: fmt::Display>(p: Point<T>) {
    println!("our point: {}", p);
}

fn print_point2<T>(p: Point<T>) 
where T: fmt::Display {
    println!("print: x: {}, y: {}", p.x, p.y);
}


#[cfg(test)]
mod tests {

}

