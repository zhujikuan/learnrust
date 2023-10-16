use std::vec;

#[allow(unused_variables)]

fn test_vec() {
    // 创建一个空的 Vec<i32>
    let mut vec1: Vec<i32> = Vec::new();

    // 使用宏创建一个包含初始元素的 Vec
    let mut vec2 = vec![1, 2, 3];

    // 向 Vec 中添加元素
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);

    let mut vec3 = Vec::new();
    vec3.push(1); // vec3可以不指定泛型类型，因为根据这一句可以推导

    // 通过索引访问 Vec 中的元素
    let second_element = vec1[1];
    println!("The second element is {}", second_element);

    // 遍历 Vec 中的元素
    for element in &vec1 {
        println!("{}", element);
    }

    // 为了修改元素内容
    for element in &mut vec2 {
        *element += 1;
    }

    // 修改 Vec 中的元素
    vec2[0] = 4;

    // 删除 Vec 中的元素
    vec2.remove(2);

    // 获取 Vec 的长度
    let vec2_length = vec2.len();
    println!("The length of vec2 is {}", vec2_length);

    // 检查 Vec 是否为空
    if vec1.is_empty() {
        println!("vec1 is empty");
    } else {
        println!("vec1 is not empty");
    }

    // 使用 Vec 的方法进行排序
    vec2.sort();
    println!("vec2 after sorting: {:?}", vec2);
}
fn test_string() {
    // 创建一个空的 String
    let mut string1 = String::new();

    // 使用字符串字面量创建一个 String
    let string2 = String::from("hello");

    // 使用 format! 宏创建一个 String
    let string3 = format!("{} {}", "hello", "world");

    // 向 String 中添加字符
    string1.push('h');
    string1.push('e');
    string1.push('l');
    string1.push('l');
    string1.push('o');

    // 向 String 中添加字符串
    string1.push_str(", world!");

    // 使用索引访问 String 中的字符
    //let first_char = string1[0]; // 这样是不行的，因为 Rust 不知道应该返回什么类型
    let first_char = string1.chars().nth(0).unwrap();
    println!("The first character is {}", first_char);

    // 使用切片访问 String 中的子串
    let hello = &string1[0..5];
    println!("The first five characters are {}", hello);

    // 连接两个 String
    let string4 = string2 + " world!";
    println!("string4 = {}", string4);

    // 使用 String 的方法进行查找和替换
    let replaced = string4.replace("world", "Rust");
    println!("replaced = {}", replaced);

    // 获取 String 的长度
    let string1_length = string1.len();
    println!("The length of string1 is {}", string1_length);

    // 检查 String 是否为空
    if string1.is_empty() {
        println!("string1 is empty");
    } else {
        println!("string1 is not empty");
    }
}

fn test_hashmap() {
    use std::collections::HashMap;
    // 创建一个空的 HashMap
    let mut map1: HashMap<i32, &str> = HashMap::new();

    // 向 HashMap 中添加元素
    map1.insert(1, "one");
    map1.insert(2, "two");
    map1.insert(3, "three");

    // 通过键访问 HashMap 中的元素
    let value = map1.get(&2);
    println!("The value of key 2 is {:?}", value);

    // 遍历 HashMap 中的元素
    for (key, value) in &map1 {
        println!("{}: {}", key, value);
    }

    // 修改 HashMap 中的元素
    map1.insert(1, "ONE");

    // 删除 HashMap 中的元素
    map1.remove(&2);

    // 获取 HashMap 的长度
    let map1_length = map1.len();
    println!("The length of map1 is {}", map1_length);

    // 检查 HashMap 是否为空
    if map1.is_empty() {
        println!("map1 is empty");
    } else {
        println!("map1 is not empty");
    }
}

fn main() {
    test_vec();
    test_string();
    test_hashmap();
}
