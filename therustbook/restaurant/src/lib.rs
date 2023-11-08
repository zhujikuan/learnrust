mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        use super::hosting;

        fn take_order() {
            // super关键字
            super::hosting::add_to_waitlist();
            hosting::add_to_waitlist();
        }
        fn serve_order() {
            self::take_order();
        }
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 在库crate中，不能使用crate name作为绝对路径的开始
    //restaurant::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
