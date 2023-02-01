
fn main() {
    let x = add(1,5);
    println!("{}", x);
}

fn add(int1: i32, int2: i32) -> i32 {
    int1 + int2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = 5;
        let y = add(x,x);
        assert_eq!(y, x+x);
    }

}