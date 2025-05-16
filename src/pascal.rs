
fn pascals_triangle(n:usize) -> Vec<Vec<usize>> {
    let mut result:Vec<Vec<usize>> = vec![];
    if n==0 {return result;}
    let mut row = vec![0;n];
    let mut  i = 0;
    row[0]=1;
    while i < n {
        for j in (1..=i).rev() {
            row[j] += row[j-1];
        }
        result.push(row[0..=i].to_vec());
        i+=1;
    }
    result
}
// fn pascals_triangle(n:usize) -> Vec<Vec<usize>> {
//     let mut result:Vec<Vec<usize>> = vec![];
//     let mut  i = 0;
//     let mut row = vec![0;n];
//     while i < n {
//         row[0]=1;
//         row[i]=1;
//         for j in 0..i/2 {
//             println!("({}, {}), row {:?}",i,j,row);
//             if i-j-1 != j+1 {
//                 row[i-j-1] += row[j+1];
//                 println!("({}, {}), row {:?}",i,j,row);
//             }
//             row[j+1] += row[j];
//             println!("({}, {}), row {:?}",i,j,row);
//         }
//         result.push(row[0..=i].to_vec());
//         i+=1;
//     }
//     result
// }

fn main() {
    println!("Test {:?}", pascals_triangle(6));
    // println!("Test {:?}", pascals_triangle(10));
    // println!("Test {:?}", pascals_triangle(4));
    // println!("Test {:?}", pascals_triangle(5));
    // println!("Test {:?}", pascals_triangle(6));
    // assert_eq!(2, psplit("badab"));
    // assert_eq!(1, psplit("dadab"));
    // assert_eq!(0, psplit("dadad"));
}

// fn pascals_triangle(n:usize) -> Vec<Vec<usize>> {
//     let mut result:Vec<Vec<usize>> = vec![];
//     let mut prev = vec![];
//     let mut  i = 0;
//     while i <= n {
//         let mut row = vec![0;i];
//         if i >= 1 {
//             row[0]=1;
//             row[i-1]=1;
//         }
//         for j in 1..(i+1)/2 {
//             row[i-j-1] = prev[j-1] + prev[j];
//             row[j] = prev[j-1] + prev[j];
//         }
//         result.push(row.clone());
//         prev = row;
//         i+=1;
//     }
//     result

// }
