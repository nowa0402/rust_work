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

    // 参照渡し
    let s2 = String::from("world");
    // &s2でオブジェクトの参照先だけ渡す。所有権はムーブしない。
    let s2_len = calculate_length(&s2);
    println!("s2,: {} s2_len: {}", s2, s2_len);

    // 可変な参照
    let mut s3 = String::from("hello");
    change_str(&mut s3);
    println!("s3: {}", s3);

    let mut hello = String::from("hello world");
    let first = first_word(&hello[..]);
    let first = first.len();
    // error (firstがhelloを参照している関係[スライス]になっているため、変更したらエラーが起きる)
    hello.clear();
    println!("word: {}", first);

    // let st = "sample hello";
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

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(", string")
}

// 与えられた文字列から最初の単語の数を返却する
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // i = 添字 item = 要素
    for (i, &item) in bytes.iter().enumerate() {
        println!("item: {}", i);
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
