// my_program/foo.rs
mod bar;
pub use self::bar::Bar;
pub fn do_foo() {
    println!("Hi from foo!");
}
