fn main() {
    println!("{}", modern_fibonacci(10));
}

fn fibonacci(n: u64) -> u64 {
    if (n == 0) {
        0
    } else if (n == 1) {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn modern_fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        (a, b) = (b, a + b)
    }

    a
}
