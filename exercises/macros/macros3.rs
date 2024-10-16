// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE
// use this 
#[macro_use]
mod macros {
    // or use this, use any one or both will work
    #[macro_export]  
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

// should not use this import, because macro is not defined in other crates
// use macros::my_macro;

fn main() {
    my_macro!();
}
