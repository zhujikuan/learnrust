use std::path::Path;

fn main() {
    // 创建一个路径对象
    let path = Path::new("/usr/bin/rustc");

    // 获取路径的文件名
    let file_name = path.file_name().unwrap();
    println!("File name: {:?}", file_name);

    // 获取路径的父目录
    let parent_dir = path.parent().unwrap();
    println!("Parent directory: {:?}", parent_dir);

    // 获取路径的扩展名
    let extension = path.extension().unwrap();
    println!("Extension: {:?}", extension);

    // 将路径转换为字符串
    let path_str = path.to_str().unwrap();
    println!("Path as string: {}", path_str);
}