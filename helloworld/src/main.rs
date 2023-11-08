


fn main(){
    let mut s = String::from("hello world");
    let s2: &mut String = &mut s; 
    let mut s3 = &s;

    //println!("s2: {}", s2);


    let s1 = &s; // 不可变引用
    let s3 = &s; // 可以创建多个不可变引用

}