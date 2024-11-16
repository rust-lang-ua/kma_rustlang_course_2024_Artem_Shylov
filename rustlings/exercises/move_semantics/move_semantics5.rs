// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;

    let y = &mut x;
    *y += 100; // Використовуємо `y` для модифікації `x`

    let z = &mut x;
    *z += 1000; // Потім використовуємо `z` для модифікації `x`

    assert_eq!(x, 1200);
}
