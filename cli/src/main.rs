
fn main() {
    let mut i = 5;
    
    if i < 10 {
        println!("i is less then 10!");
    } else if i == 11 {
        println!("i is equal to 11!")
    } else if i != 100 {
        println!("i is not 100");
    } else {
        println!("i is 100");
    }
    
    let b = if i > 10 {1} else {5};
    
    println!("b: {}", b);
    
    match i {
        1 => println!("i is one"),
        2 => {
            println!("2 is for multi line!");
            println!("awesome!");
        },
        _ => println!("was not a one or a two"),
    }
    
    while i > 0 {
        println!("i is {}", i);
        i -= 1;
    };
    
    loop {
        break;
    }
}
      

#[cfg(test)]
mod tests {
   

}