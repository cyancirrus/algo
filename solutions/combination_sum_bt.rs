fn subsets_uniq(elems: &mut [usize]) -> Vec<Vec<usize>> {
    elems.sort_unstable();
    let mut sols = Vec::with_capacity(1 << elems.len());

    fn bt(start: usize, elems: &[usize], coll: &mut Vec<usize>, sols: &mut Vec<Vec<usize>>) {
        sols.push(coll.clone());
        for i in start..elems.len() {
            if i > 0 && elems[i] == elems[i-1] { continue; }
            coll.push(elems[i]);
            bt(i+1, elems, coll, sols);
            coll.pop();
        }
    }

    bt(0, elems, &mut vec![], &mut sols);
    sols
}


fn subsets_two(elems: &mut [usize]) -> Vec<Vec<usize>> {
    elems.sort_unstable();
    let mut sols = Vec::with_capacity(1 << elems.len());

    fn bt(start: usize, elems: &[usize], coll: &mut Vec<usize>, sols: &mut Vec<Vec<usize>>) {
        sols.push(coll.clone());
        for i in start..elems.len() {
            coll.push(elems[i]);
            bt(i+1, elems, coll, sols);
            coll.pop();
        }
    }

    bt(0, elems, &mut vec![], &mut sols);
    sols
}

fn subsets(elems:&[usize]) -> Vec<Vec<usize>> {
    let n = elems.len();
    let mut sols = Vec::with_capacity(1<<n); 
    fn bt(start:usize, elems:&[usize], coll:&mut Vec<usize>, sols:&mut Vec<Vec<usize>>) {
        sols.push(coll.clone());
        for i in start..elems.len() {
            coll.push(elems[i]);
            bt(i+1, elems, coll, sols);
            coll.pop();
        }
    }
    bt(0, elems, &mut vec![], &mut sols);
    sols
}


fn main() {
    // println!("subsets {:?}", subsets(&[1,2,3]));
    println!("subsets {:?}", subsets_uniq(&mut vec![1,1,1,1,2,3]));
}
