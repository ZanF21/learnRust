//fib0 =1. fib1 = 1, fib2 = 2
fn main() {
    println!("Enter n : ");
    let mut n=String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");
    let n:u32 = n.trim().parse().expect("Please type a number!");
    println!("Fibonacci number is {}",fibn(n));
}

fn fibn(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        let mut f2=1;
        let mut f1=1;
        let mut f=0;
        for _i in 2..n+1 {
            f = f1 + f2;
            f2 = f1;
            f1 = f;
        }
        f
    }
}