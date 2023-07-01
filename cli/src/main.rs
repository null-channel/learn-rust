use std::{error::Error, fmt};


struct PointOld {
    x: i32,
    y: i32,
}

impl fmt::Display for PointOld {
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
    let x = 5;
    let old_point = PointOld {x: 5, y: 10};
    let new_point: Point<String> = Point {x: 5, y: 10};
    println!("new point: {}", new_point);
    let old_point_2 = PointOld {x: 2, y: 2};
    let new_point_2 = Point {x: old_point, y: old_point_2};
    println!("point of PointOlds: {}", new_point_2);
    print_point(new_point_2);
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

