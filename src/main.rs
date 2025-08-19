// fn reverse_bits(mut n:u32) -> u32 {
//     const B = u32::BITS;
//     for pos in 0..B/2 {
//         let high = n >> (B - pos);
//         let low = (n >> pos) & 1;
//         n -= high << (B - pos) - low << (pos);
//         n += low << (B -pos) + high << pos;
//     }
//     n
// }


fn one_bits(mut x:u8) -> u8 {
    let mut bits = 0;
    for s in 0..u8::BITS {
        if x == 0 { return bits; }
        bits += (x >> s) & 1;
    }
    bits
}

fn reverse_bits(mut x: u8) -> u8 {
    x = (( x >> 1 ) & 0b0101_0101) | ((x & 0b0101_0101) << 1);
    x = (( x >> 2 ) & 0b0011_0011) | ((x & 0b0011_0011) << 2);
    x = ( x >> 4 ) | (x << 4);
    x
}


// fn reverse_bits(mut n:u8) -> u8 {
//     const B:u8 = u8::BITS  as u8 - 1;
//     for pos in 0..B/2 {
//         let high = n >> (B - pos) & 1;
//         let low = (n >> pos) & 1;
//         if high != low {
//             n ^=  (1 << (B - pos)) | (1 << pos) ;
//         }
//     }
//     n
// }



// fn reverse_bits(mut n:u8) -> u8 {
//     const B:u8 = u8::BITS  as u8 - 1;
//     for pos in 0..B/2 {
//         let high = n >> (B - pos) & 1;
//         let low = (n >> pos) & 1;
//         n = n 
//             - (high << (B - pos)) - (low << pos)
//             + (low << (B - pos)) + (high << pos)
//         ;
//     }
//     n
// }


fn main() {
    println!("one bits {:?}", one_bits(5));
    println!("test {:x}", 0b1111_0000);
    // println!("test {:b}", 0x33);
    let mut x:u8 = 0b1010_0000;
    x = reverse_bits(x);
    println!("x {x:b}", );
}
