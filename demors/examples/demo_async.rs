// 定义一个异步函数
async fn hello_world() {
    println!("Hello, world!");
}

// 在主函数中运行异步函数
async fn main() {
    let a = hello_world();
    println!("a = {:?}", a);
    task::block_on(hello_world());
}
