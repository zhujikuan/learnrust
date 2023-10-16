#[allow(unused_variables)]
fn test_enum() {
    // 枚举类型
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 枚举类型的变体,为枚举绑定数据
    #[derive(Clone, Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 枚举类型的方法
    impl IpAddr {
        fn call(&self) {
            println!("call");
        }
    }

    home.call();

    // 取出枚举类型的数据
    match home {
        IpAddr::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => println!("{}", s),
    }
}

fn test_option() {
    // Option<T>类型
    let x: Option<i32> = Some(5);
    let y: Option<f64> = None;

    //
    match x {
        Some(i) => println!("i = {}", i),
        None => println!("i = None"),
    }

    match y {
        Some(f) => println!("f = {}", f),
        None => println!("f = None"),
    }
}

fn test_result(){
    // Result<T, E>类型 ,也是个特殊的枚举
    let myerr:Result<String,String> = Result::Err(String::from("my error"));
    let myok:Result<String,String> = Result::Ok(String::from("my ok"));
    
    match myerr {
        Result::Err(e) => println!("e = {}",e),
        Result::Ok(o) => println!("o = {}",o),
    }

    match myok {
        Result::Err(e) => println!("e = {}",e),
        Result::Ok(o) => println!("o = {}",o),
    }
}

fn main() {
    test_enum();
    test_option();
    test_result();
}
