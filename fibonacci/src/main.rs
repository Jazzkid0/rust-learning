fn main() {
    fib(0, 1)
}

fn fib(x:i32, y:i32) {
    println!("{}", x);
    if y < 100 {
        fib(y, x + y);
    }
}
