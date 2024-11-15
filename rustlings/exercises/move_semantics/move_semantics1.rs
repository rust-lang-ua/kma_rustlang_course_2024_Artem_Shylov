// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // `vec0` is moved, and `vec1` now owns it

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88); // This works because `vec1` owns the vector now

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
