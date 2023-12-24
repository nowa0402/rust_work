// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 絶対パス
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相対パス
//     front_of_house::hosting::add_to_waitlist();
// }

// use文
// 絶対パス
// use crate::front_of_house::hosting;
// 相対パス
// use self::front_of_house::hosting;

// pub fn eat_at_restaurant_use() {
//     hosting::add_to_waitlist();
// }

fn serve_order() {}

mod back_of_house {
    // super
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    // クラス定義
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

pub fn eat_at_restaurant_a() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // コンパイルエラー
    // meal.seasonal_fruit = String::from("blueberries");
}

pub mod foo {
    pub fn foofn() {}
}

mod bar {
    // モジュールの再公開
    pub use crate::foo;
}

mod baz {
    // bar::fooでfooが呼び出せるようになる
    use crate::bar::foo;
    fn bazfn() {
        foo::foofn()
    }
}

// 他のパッケージを使用する
mod front_ob_house;

pub use crate::front_ob_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist()
}
