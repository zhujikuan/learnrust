fn test_chars_iter() {
    let s = String::from("hello");
    let mut iter = s.chars();

    match iter.nth(2) {
        Some(c) => println!("2: {}", c),
        None => println!("None"),
    }

    while let Some(c) = iter.next() {
        println!("{}", c);
    }

    println!("cnt: {}", s.chars().count());

    let v: Vec<char> = s.chars().collect();
    println!("v: {:?}", v);
}

#[warn(dead_code)]
fn test_range() {
    let r1 = 1..5;
    let r2 = 1..=5;
    for i in r1 {
        println!("{}", i);
    }
    for i in r2 {
        println!("{}", i);
    }
}
fn main() {
    //test_range();
    test_chars_iter();
}
