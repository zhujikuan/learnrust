#![allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test_box() {
    let b = Box::new(5);
    println!("b = {}", b);
    let p = Box::new(Point { x: 1, y: 2 });
    println!("p = {:?}", p);
    //Box<T>，可用于在堆上分配值, 适用于如下场景：
    //1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
    
    //2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
    //3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

}

fn main() {
    test_box();
}
