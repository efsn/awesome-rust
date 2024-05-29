mod customer;

use crate::{back_of_house, front_of_house};

pub fn eat_at_restaurant() {
    // 订购黑麦吐司当早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 更改面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}