// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// method 1
// fn main() {
//     let vec0 = Vec::new();
//
//     let mut vec1 = fill_vec(vec0.clone());
//
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
//
//     vec1.push(88);
//
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }
//
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;
//
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//
//     vec
// }

//method 2 (not sure if this is what they want)
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(_vec: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();

    res.push(22);
    res.push(44);
    res.push(66);

    res
}

//method 3
// fn main() {
//     let mut vec0 = Vec::new();
//     fill_vec(&mut vec0);
//
//     // let mut vec1 = fill_vec(&mut vec0);
//
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
//
//     // vec1.push(88);
//
//     // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }
//
// fn fill_vec(vec: &mut Vec<i32>) {
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// }
