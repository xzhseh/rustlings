// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.


fn main() {
    my_macro!();
}

#[macro_export] // Or just move this macro to the above of the main() function
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
