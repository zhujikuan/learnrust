// Title: 线程
fn test_thread() {
    let process_worker = || {
        for i in 1..5 {
            println!("hi this worker thread: num={}", i);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    };
    // 创建线程
    let handle = std::thread::spawn(process_worker);

    // 主线程
    for i in 1..5 {
        println!("hi this is master,num={}", i);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // 等待线程结束
    handle.join().unwrap();
}

fn test_thread2() {
    let handle = std::thread::spawn(|| {
        for i in 1..10 {
            println!("hi this is thread2,num={}", i);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi this is master,num={}", i);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn test_channel() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn test_mutex() {
    let m = std::sync::Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn main() {
    //test_thread();
    //test_thread2();
    // test_channel();
    test_mutex();
}
