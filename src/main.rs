fn main() {
    // let mut x = [2,4,1,3];
    // merge_heap(0, 2, 3, &mut x);
    // println!("X {x:?}");
    // let mut x = [1,2,3,4,5,6,7,8];
    // merge_heap(0, 4, 7, &mut x);
    // println!("X {x:?}");
    let mut x = [2,4,1,3];
    merge_ring(0, 2, 3, &mut x);
    println!("X {x:?}");
    let mut x = [1,2,3,4,5,6,7,8];
    merge_ring(0, 4, 7, &mut x);
    println!("X {x:?}");
}
