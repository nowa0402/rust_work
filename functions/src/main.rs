fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };
    println!("y value: {}", y);

    let x = five();
    println!("x value: {}", x);

    let x = plus_one(x);
    println!("x: {}", x);

    let number = 6;

    // if文
    if number % 4 == 0 {
        println!("OK!");
    } else {
        println!("NG!");
    }

    // 三項演算子?
    let number = if number == 6 { 5 } else { 0 };
    println!("number {}", number);

    // for
    for elem in 1..10 {
        println!("num: {}", elem);
    }

    println!();

    // reverse
    for elem in (1..10).rev() {
        println!("num: {}", elem);
    }

    // 所有権操作
    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("s1: {}", s1); // error
}

// 引数定義
fn another_function(x: i32) {
    println!("x is {}", x);
}

// 戻り値あり
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn take_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}
