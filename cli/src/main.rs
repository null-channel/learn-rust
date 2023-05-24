use std::{error::Error, fmt};

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn new_default() -> Self 
    where T: Default {
        Self { x: T::default(), y: T::default() }
    }

    fn update(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

/*
impl fmt::Display for Point<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }    
}
*/
/*
impl<T> fmt::Display for Point<T> {
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


struct OldPoint {
    x: i32,
    y: i32,
}

impl fmt::Display for OldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }    
}

fn main() -> Result<(),Box<dyn Error>> {

    let p = OldPoint { x: 5, y: 10 };
    let p2 = Point { x: 5, y: 10 };
    let p3 = Point { x: 5.0, y: 10.0 };
    println!("p3: {}", p2);
    let p4 = Point { x: "Hello", y: "World" };
    //let p5 = Point::new_default();
    let p6: Point<i32> = Point::new_default();
    let p7 = Point::<i32>::new_default();
    let mut p8 = Point::new_default();
    p8.update(1, 5);

    println!("p8: {}", p8);

    print_point(p2);

    let mut hm = std::collections::HashMap::new();
    hm.insert("Hello", "World");

    Ok(())
}

fn print_point<T: fmt::Display>(p: Point<T>)  {
    println!("print: {}", p);
}

fn print_point2<T>(p: Point<T>) 
where T: fmt::Display {
    println!("print: x: {}, y: {}", p.x, p.y);
}



#[cfg(test)]
mod tests {
   

}