// fn count_and_say(nums:&str) -> String {
//     let len = nums.len();
//     let chars: Vec<char> = nums.chars().collect();
//     let mut compress = String::new();
//     let mut count= 1;
//     if len == 0 { return compress }

//     for i in 1..len {
//         if chars[i] == chars[i-1] {
//             count +=1
//         } else {
//             compress.push_str(&count.to_string());
//             compress.push(chars[i-1]);
//             count = 1;
//         }
//     }
//     compress.push_str(&count.to_string());
//     compress.push(chars[len - 1]);
//     compress
// }

fn count_and_say(nums:&str) -> String {
    let mut chars = nums.chars();
    let mut compress = String::new();
    if let Some(mut prev) = chars.next() {
        let mut count = 1;
        for c in chars {
            if c == prev {
                count += 1;
            } else {
                compress.push_str(&count.to_string());
                compress.push(prev);
                prev = c;
                count = 1;
            }
        }
        compress.push_str(&count.to_string());
        compress.push(prev);
    }
    compress
}


fn main() {
    assert_eq!("1234", count_and_say("2444"));
    assert_eq!("331221", count_and_say("333211"));
}
