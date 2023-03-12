fn main() {
    let n = 9;
    let res = factorial(n);

    println!("{}", res);
}

fn factorial(a: u32) -> u32 {
    if a == 1 {
        return 1;
    } else {
        return factorial(a-1) * a;
    }
}

fn whatTheFuck() {
    return 0;
}
