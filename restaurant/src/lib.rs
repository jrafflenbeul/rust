use std::collections::HashMap;
use std::fmt;
// use std::io;
use std::fmt::Result;
use std::io::Result as IoResult;
// use multiple at once
// use std::{cmp::Ordering, io};
// self == io
use std::io::{self, Write};
// use everything
use std::collections::*;

// moved to src/front_of_house.rs
/*mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}*/

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod front_of_house;

// use crate::front_of_house::hosting;
// re-exporting
pub use crate::front_of_house::hosting;

// relative use path
// use self::front_of_house::hosting;

// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use path
    hosting::add_to_waitlist();

    // direct access through use
    // add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // won't compile
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}