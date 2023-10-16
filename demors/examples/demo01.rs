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

fn test_slice() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[0..2]; // 借用arr的一部分，slice是一个slice
    println!("arr = {:?}", arr); // arr还可以使用
    println!("slice = {:?}", slice);

    let arr = [1, 2, 3, 4, 5];

    // 从索引 0 开始，到索引 2 结束（不包括 2）
    let slice1 = &arr[0..2];
    println!("slice1 = {:?}", slice1); // 输出 [1, 2]

    // 从索引 0 开始，到索引 2 结束（不包括 2），可以省略 0
    let slice2 = &arr[..2];
    println!("slice2 = {:?}", slice2); // 输出 [1, 2]

    // 从索引 1 开始，到数组结尾
    let slice3 = &arr[1..];
    println!("slice3 = {:?}", slice3); // 输出 [2, 3, 4, 5]

    // 从索引 0 开始，到数组结尾，可以省略 0 和数组长度
    let slice4 = &arr[..];
    println!("slice4 = {:?}", slice4); // 输出 [1, 2, 3, 4, 5]

    // 从索引 0 开始，到索引 5 结束（超出数组长度不会报错）
    let slice5 = &arr[0..5];
    println!("slice5 = {:?}", slice5); // 输出 [1, 2, 3, 4, 5]

    // 从索引 0 开始，到索引 6 结束（超出数组长度不会报错），可以省略数组长度
    let slice6 = &arr[..6];
    println!("slice6 = {:?}", slice6); // 输出 [1, 2, 3, 4, 5]

    let a = "abc".to_string();
    let b = &a[0..1]; // 借用a的一部分，b是一个slice
    println!("a = {}", a); // a还可以使用
    println!("b = {}", b);
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
    test_slice();
}
