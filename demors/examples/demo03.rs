#[allow(unused_variables)]

fn test_struct01() {
    struct Point {
        // 正常结构体
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);

    struct Position(i32, i32); // 元组结构体
    let p = Position(1, 2);
    println!("p.0 = {}", p.0);
    println!("p.1 = {}", p.1);

    struct Unit; // 单元结构体
    let u = Unit;
}

fn test_struct02() {
    struct Student {
        name: String,
        age: i32,
    }
    let s1 = Student {
        name: "张三".to_string(),
        age: 20,
    };
    let s2 = Student {
        name: "李四".to_string(),
        ..s1 // 剩余字段使用s1填充
    };
    println!("s2.name = {}", s2.name);
    println!("s2.age = {}", s2.age);
}

// fn test_method(){
//     struct Student {
//         name: String,
//         age: i32,
//     }
//     impl Student {
//         fn print(&self) {
//             println!("name = {}", self.name);
//             println!("age = {}", self.age);
//         }
//     }
// }

fn main() {
    test_struct01();
    test_struct02();
    //test_method();
}
