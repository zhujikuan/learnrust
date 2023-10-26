fn test_declarative_macro() {
    macro_rules! mymacro {
        () => {
            println!("Hello, macro!");
        };
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
    }

    mymacro!();
    mymacro!("world");
}

fn main() {
    test_declarative_macro();
}
