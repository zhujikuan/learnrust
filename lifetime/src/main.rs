#[derive(Debug)]
struct Number<'a> {
    num: &'a u8,
}

impl<'a> Number<'a> {
    fn get_num(&self) -> &u8 {
        self.num
    }
    fn set_num(&mut self, num: &'a u8) {
        self.num = num;
    }
}
fn main() {
    let a = 10;
    let mut num = Number { num: &a };
    num.set_num(&100);
    println!("{:?}", num.get_num());
}
