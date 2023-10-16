#[allow(unused_variables)]

fn test_iter() {
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

    let mut counter = Counter::new();
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter.next());
    println!("counter = {:?}", counter);

    for i in Counter::new() {
        println!("i = {}", i);
    }


}

fn main() {
    test_iter();
}
