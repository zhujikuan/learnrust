fn test_unwrap() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    // 使用 unwrap 方法获取 Some 中的值
    println!("x = {}", x.unwrap());

    // 使用 unwrap 方法触发 panic
    // println!("y = {}", y.unwrap());

    // 使用 unwrap_or 方法获取 Some 中的值
    println!("x = {}", y.unwrap_or(10));
}
fn test_question() -> Result<(), String> {
    let x: Result<i32, String> = Err("this is an error".to_string());
    let y = x?;
    println!("y = {}", y);
    Ok(())
}

fn main() {
    test_unwrap();
    let res = test_question();
    println!("res = {:?}", res);
}
