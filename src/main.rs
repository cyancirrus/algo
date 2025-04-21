use prepatory::memory::List;

fn custom_adder(x:usize) -> impl Fn(usize) -> usize {
    move |y| x + y
}

fn main() {
    let x = 32;
    let adder = custom_adder(x);
    println!("hello world");

    println!("Should be 64 {}", adder(32));
}
