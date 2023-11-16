fn main() {
    // match
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    // if let
    let x = Some(5);
    if let Some(5) = x {
        println!("x is five");
    } else {
        println!("x is not five");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // let
    let (a, b, c) = (1, 2, 3);

    // 模式语法
    // 匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
    // 匹配命名变量
    let x = Some(5);
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // 多重模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }

    // 使用...来匹配区间
    let x = 5;
    match x {
        1..=4 => println!("one through four"),
        _ => println!("anything"),
    }

    // 使用解构来分解值
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 使用_,忽略模式中的值
    let (_, b, c) = (1, 2, 3);
    // 使用..忽略值的剩余部分
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    // 使用匹配守卫来添加额外条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // @绑定
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
