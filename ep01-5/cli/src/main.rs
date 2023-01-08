use rand::Rng;

fn main() {
    let n1 = random_number();
    let n2 = random_number();
    let n3 = n1 + n2;
    println!("{} + {} = {}", n1, n2, n3);
}

fn random_number() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100, 255)
}
