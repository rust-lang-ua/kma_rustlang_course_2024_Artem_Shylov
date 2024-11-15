// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {

    pub fn get_secret_recipe() -> String {
        String::from("Ginger")
    }


    pub fn make_sausage() {
        let recipe = get_secret_recipe();
        println!("Making sausage with: {}", recipe);
    }
}

fn main() {
    sausage_factory::make_sausage();
}
