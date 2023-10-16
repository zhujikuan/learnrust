// Title: 线程
fn test_thread() {
    use std::thread;
    use std::time::Duration;

    let process_worker = || {
        for i in 1..5 {
            println!("hi this worker thread: num={}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    };
    // 创建线程
    let handle = thread::spawn(process_worker);

    // 主线程
    for i in 1..5 {
        println!("hi this is master,num={}", i);
        thread::sleep(Duration::from_millis(1000));
    }

    // 等待线程结束
    handle.join().unwrap();
}

fn main() {
    test_thread();
}
