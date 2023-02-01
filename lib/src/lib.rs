
/// # Examples
/// ```
/// # use lib::add;
/// # let x = 5;
/// let y = add(x,x);
/// # assert_eq!(y,x + x);
/// ```
pub fn add(left: usize, right: usize, middle: usize) -> usize {
    left + right + middle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
