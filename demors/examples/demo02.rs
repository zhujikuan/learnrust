#[allow(unused_variables)]
fn test_ownership() {
    let a = "abc".to_string();
    {
        let b = a; // move, a的所有权转移到b
                   //println!("a = {}", a); 所有权转移后，a就不能再使用了，a被无效化了
        println!("b = {}", b);
    }
    //println!("b = {}", b); b离开作用域，被编译器调用drop了
}

fn test_ref() {
    let a = "abc".to_string();
    let b = &a; // borrow, a的所有权没有转移，b只是借用了a
    println!("a = {}", a); // a还可以使用
    println!("b = {}", b);

    let count = 1;
    let mut _a = &count;
    // cannot mutably borrow immutable local variable `count`
    // let mut _b = &mut count;
    let mut count = 2;
    let mut _a = &mut count;
    let mut _b = &count;
    let mut _c = &mut count;
}
fn main() {
    test_ownership();
    test_ref();
}
