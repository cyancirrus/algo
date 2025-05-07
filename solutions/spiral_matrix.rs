// fn spiral_matrix(n:usize) -> Vec<usize> {
//     let mut m = vec![0; n*n];
//     let mut c = 1;
   
//     m[ (n * n ) / 2 ] = n * n;
//     // should iterate for only one approach
//     for i in 0..(n + 1) / 2 {
//         // should terminate at inner matrix size 1 or inner matrix size 2
//         for j in i..n - i - 1{
//             println!("i: {}, j: {}", i, j);
//             // [left upper, right upper, right down, left down]
//             let lu = i * n + j;
//             let ru = (1 + i + j) * n - 1;
//             let rd = (n - i) * n - j - 1;
//             let ld = (n - i - j - 1) * n + i;
//             println!("lu:{}, ru:{}, rd:{}, ld:{}", lu,ru, rd, ld);
//             m[lu] = c + j;
//             m[ru] = c + j + (n-1);
//             m[rd] = c + j + 2 * (n-1);
//             m[ld] = c + j + 3 * (n-1);
//         }
//         println!("Result {:?}", m);
//         c+=4*(n - 1);

//     }
//     println!("End Result {:?}", m);
//     m 
// }

fn spiral_matrix(n: usize) -> Vec<usize> {
    let mut matrix = vec![0; n * n];
    let mut count = 1;

    // Pre-fill the center for odd-sized matrices
    if n % 2 == 1 {
        matrix[(n * n) / 2] = n * n;
    }

    // Fill each layer of the spiral
    for layer in 0..(n + 1) / 2 {
        for offset in layer..n - layer - 1 {
            let top_left     = layer * n + offset;
            let top_right    = (offset ) * n + n - 1 - layer;
            let bottom_right = (n - layer) * n - offset - 1;
            let bottom_left  = (n  - offset - 1) * n + layer;
            
            matrix[top_left]     = count + offset;
            matrix[top_right]    = matrix[top_left] + (n - 1  - 2*layer);
            matrix[bottom_right] = matrix[top_right] + (n - 1 -  2*layer);
            matrix[bottom_left]  = matrix[bottom_right] + (n - 1 - 2*layer);
        }
        if n > 1 {
            count = matrix[(layer + 1) * n  + 2 * layer];
        }
        println!("count {}", count);
    }
    matrix
}





// a a a a
// a a a a
// a a a a
// a a a a
//
// b b b
// b b b
// b b b
//
// c c
// c c
//
// d

fn main() {
    // spiral_matrix(3);
    assert_eq!(vec![1], spiral_matrix(1)); 
    assert_eq!(vec![1,2,4,3], spiral_matrix(2)); 
    assert_eq!(vec![1,2,3,8,9,4,7,6,5], spiral_matrix(3)); 
}
