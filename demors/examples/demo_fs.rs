use std::fs;
use std::io::BufRead;
use std::thread;
use std::time::Duration;

fn test_get_file_info() {
    // 获取文件信息
    let metadata = fs::metadata("Cargo.toml");
    let exist = metadata.is_ok();
    println!("Cargo.toml exist: {}", exist);
    // 打印所有metadata中，可读的属性
    if let Ok(attr) = metadata {
        println!("is_dir: {}", attr.is_dir());
        println!("is_file: {}", attr.is_file());
        println!("len: {}", attr.len());
        println!("modified: {:?}", attr.modified());
        println!("created: {:?}", attr.created());
        println!("accessed: {:?}", attr.accessed());
    }
}

fn test_file_opt() {
    // 创建文件
    let file = fs::File::create("test.txt");
    #[allow(unused_variables)]
    if let Ok(f) = file {
        println!("create file success");
    }
    thread::sleep(Duration::from_secs(5));

    //重命名
    let result = fs::rename("test.txt", "test2.txt");
    if let Ok(()) = result {
        println!("rename file success");
    }
    thread::sleep(Duration::from_secs(5));

    // 移动文件
    let result = fs::rename("test2.txt", "src/test2.txt");
    if let Ok(()) = result {
        println!("move file success");
    }
    thread::sleep(Duration::from_secs(5));

    // 删除文件
    let result = fs::remove_file("src/test2.txt");
    if let Ok(()) = result {
        println!("remove file success");
    }
}

fn test_file_rw() {
    // 打开文件
    let file = fs::File::open("Cargo.toml");
    #[allow(unused_variables)]
    // match file {
    //     Ok(f) => {
    //         println!("open file success");
    //     }
    //     Err(e) => {
    //         println!("open file failed, err: {}", e);
    //     }
    // }
    if let Ok(f) = file {
        println!("open file success");
        let reader = std::io::BufReader::new(f);
    }
}

fn main() {
    test_get_file_info();
    test_file_opt();
    test_file_rw();
}
