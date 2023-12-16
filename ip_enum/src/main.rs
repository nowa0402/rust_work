// enum定義
#[derive(Debug)]
// #[warn(dead_code)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

impl IpAddrKind {
    fn call(&self) {
        println!("ip: {:?}", self);
    }
}

fn get_some(x: Option<i32>) -> i32 {
    match x {
        None => 1,
        Some(i) => i,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // 列挙子のインスタンス生成
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    loopback.call();
    home.call();

    // optional
    let some_number = Some(5);
    let some_string = Some("a string");

    // Optional型定義
    // Noneを指定する時は型まで指定する
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
    // }

    // 糖衣構文
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // コンパイルエラー（Noneチェックが必須）
    // let sum = x + y;
}
