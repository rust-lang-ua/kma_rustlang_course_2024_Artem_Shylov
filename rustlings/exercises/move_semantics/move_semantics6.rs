// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Передаємо посилання, щоб не віддавати володіння

    string_uppercase(data); // Передаємо володіння, як цього вимагає функція
}

// Should not take ownership
fn get_char(data: &String) -> char { // Приймає посилання
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) { // Приймає володіння
    data = data.to_uppercase(); // Змінюємо саме значення, а не посилання

    println!("{}", data);
}
