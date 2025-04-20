fn longest_prefix(strings:Vec<&str>) -> String {
    let mut common:String = String::new(); 
    if strings.is_empty() {
        return String::new()
    }
    let first = &strings[0];
    for (i, ch) in first.chars().enumerate() {
        for word in &strings[1..] {
            if word.chars().nth(i) != Some(ch) {
                return common
            }
        }
        common.push(ch);
    }
    common
}


fn main() {
    let strs = vec!["flower","flow","flight"];
    println!("longest prefix {:?}", longest_prefix(strs));
    println!("hello world");
}
