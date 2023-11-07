use bytes::{Buf, BufMut, BytesMut};

fn test_buf() {
    let mut buf = &b"hello world"[..];
    println!("{:?}", buf.len());

    println!("{}", buf.get_u8() as char);
    println!("{}", buf.get_u8() as char);
    println!("{}", buf.get_u8() as char);
    println!("{:?}", buf.len());

    let mut rest = [1; 8];
    buf.copy_to_slice(&mut rest);
    println!("{:?}", buf.len());

    println!("{:?}", rest);
    println!("{:?}", buf);
}

fn test_bufmut() {
    let mut buf = BytesMut::with_capacity(8);
    println!("capacity: {:?}", buf.capacity());
    println!("len: {:?}", buf.len());
    println!("is_empty: {:?}", buf.is_empty());
    
    buf.put(&b"helloworld"[..]);
    println!("{}", buf.get_u8() as char);
}
fn main() {
    // test_buf();
    test_bufmut();
}
