#[allow(unused_variables)]

fn test_basic_usage() {
    // 创建一个包含初始元素的数组
    let mut arr = [1, 2, 3, 4, 5];

    println!("原始: arr = {:?}", arr);
    // 使用 for 循环遍历数组并打印每个元素
    print!("for trave: ");
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!();

    // for 可变引用
    for element in &mut arr.iter_mut() {
        *element += 10;
    }
    println!("原始: arr = {:?}", arr);

    // 使用 iter 方法创建一个迭代器并遍历它
    let mut iter = arr.iter();
    while let Some(element) = iter.next() {
        print!("{} ", element);
    }
    println!();

    // 使用 map 方法创建一个迭代器并对每个元素进行操作
    let squares: Vec<i32> = arr.iter().map(|x| x * x).collect();
    println!("{:?}", squares);

    // 使用 filter 方法创建一个迭代器并过滤元素
    let even_numbers: Vec<i32> = arr.iter().filter(|x| **x != 11).cloned().collect();
    println!("{:?}", even_numbers);
}

fn test_diy_iter() {
    // 自定义迭代器
    #[derive(Debug)]
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new();
    let mut iter = counter.into_iter();
    println!("counter = {:?}", iter.next());
    println!("counter = {:?}", iter.next());
    println!("counter = {:?}", iter.next());
    println!("counter = {:?}", iter.next());
    println!("counter = {:?}", iter.next());
    println!("counter = {:?}", iter.next());
    println!("counter = {:?}", iter.next());
    // println!("counter = {:?}", iter.next());
    // println!("counter = {:?}", iter.next());
    // println!("counter = {:?}", iter.next());

    // for i in Counter::new() {
    //     println!("i = {}", i);
    // }
}

fn main() {
    //test_basic_usage();
    test_diy_iter();
}
