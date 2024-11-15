// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)



use std::f32::consts::PI;

fn main() {
    let pi = PI; // Используем точное значение числа π
    let radius = 10.0f32;

    let circumference = 2.0 * pi * radius;

    println!(
        "The circumference of a circle with radius {:.2} is {:.5}!",
        radius, circumference
    );
}
