fn main() {
    let input = 7;
    println!("Your input was {} and output was {}.", input, nth_fibonacci(input));
}

fn nth_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}