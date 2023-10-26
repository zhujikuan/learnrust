fn main() {
    let s = "hello, world!";
    let mut chars = s.chars();

    // 使用 for 循环遍历 Chars 迭代器
    for c in chars.clone() {
        println!("{}", c);
    }

    // 使用 next 方法获取下一个字符
    let first_char = chars.next();
    println!("The first character is {:?}", first_char);

    // 使用 count 方法获取字符数
    let char_count = s.chars().count();
    println!("The string has {} characters", char_count);

    // 使用 nth 方法获取第 n 个字符
    let third_char = s.chars().nth(2);
    println!("The third character is {:?}", third_char);

    // 使用 last 方法获取最后一个字符
    let last_char = s.chars().last();
    println!("The last character is {:?}", last_char);
}
