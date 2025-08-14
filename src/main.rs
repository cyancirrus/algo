fn gray_code_review(n:u8) -> Vec<u8> {
    let mut res = Vec::with_capacity(1<<n);
    for i in 0..1<<n {
        res.push(i^ (i - 1));
        
    }
    res
}
fn main() {
    println!("gray code {:?}", gray_code_review(2));

}
