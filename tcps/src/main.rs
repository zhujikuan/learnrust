#[allow(unused_variables)]

fn test_variable() {
    let x = 5; // 一个变量，不可修改
               // x = 6; // 报错：cannot mutate immutable variable `x`
    let mut y = 6; // 一个可变的变量
    println!("y = {}", y);
    y = 7; // 可以修改
    println!("y = {}", y);
}

fn test_constant() {
    const PI: f32 = 3.1415926; // 常量，不可修改,必须指明类型
    println!("pi = {}", PI);
}

fn test_hiden() {
    let x = 1;
    println!("{:p}  {}", &x, x); // 0x7ffc38ad0dec  1
    let x = '中';
    println!("{:p}  {}", &x, x); // 0x7ffc38ad0e4c  中

    // 前后x的地址不一样，说明x是一个新的变量，称之为覆盖，甚至类型都可以改变
}

fn test_tuple() {
    let position: (u32, u32) = (100, 100);
    println!("x = {}, y = {}", position.0, position.1); // 通过索引访问
    let (x, y) = position; // 解构
    println!("x = {}, y = {}", x, y);
}

fn test_array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 大小固定
    println!("arr[0] = {}", arr[0]);
    //println!("arr[1] = {}", arr[10]); // 编译期就会报错
}

fn test_func(arg1: i32, arg2: i32) -> i32 {
    arg1 + arg2 // 没有分号，表示返回值，有分号表示语句
}

fn test_if() {
    let x = 5;
    if x == 5 {
        println!("x = 5");
    } else if x > 5 {
        println!("x > 5");
    } else {
        println!("x < 5");
    }

    let y = if x == 5 { 5 } else { 6 }; // if表达式
    println!("y = {}", y);
}

fn test_loop() {
    let mut x = 0;
    loop {
        // 类似于while(true)
        x += 1;
        if x == 5 {
            break;
        }
    }
    println!("x = {}", x);
    x = loop {
        x += 1;
        if x == 10 {
            break x * 2;
        }
    };
    println!("x = {}", x);
}

fn test_while() {
    let mut x = 0;
    while x < 5 {
        x += 1;
    }
    println!("x = {}", x);
    x = 0;
    while x < 5 {
        x += 1;
        if x == 3 {
            continue;
        }
        println!("x = {}", x);
    }
}

fn test_for() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("i = {}", i);
    }
    for i in 0..5 {
        println!("i = {}", i);
    }
}

fn main() {
    println!("Hello, world!");
    test_variable();
    test_constant();
    test_hiden();
    test_tuple();
    test_array();
    println!("test_func(1,2) = {}", test_func(1, 2));
    test_if();
    test_loop();
    test_while();
    test_for();
}
