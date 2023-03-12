fn main() {
    let fib = 15;
    let res = fibonacci(fib);

    println!("{}", res);
}

fn fibonacci(a: i32) -> i32 {
    if a < 2 {
        return a;
    } else {
        return fibonacci(a-1) + fibonacci(a-2);
    }
}
