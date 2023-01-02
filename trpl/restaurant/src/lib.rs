use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
fn serve_order() {}
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // 绝对路径引用模块
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径引用模块
    front_of_house::hosting::add_to_waitlist();
    self::serve_order();
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要的面包的类型
    meal.toast = String::from("wheat");
    println!("{}", meal.toast);
    hosting::add_to_waitlist();
    let mut map = HashMap::new();
    map.insert(1, 2);
}