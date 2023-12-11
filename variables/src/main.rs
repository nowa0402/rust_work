fn main() {
    // 3.1 変数
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        // 12
        println!("The value of x in the inner scope is: {}", x);
    }

    // 6
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("space: {}", spaces);

    // 3.2 データ型
    // 複数の型が推論される時は型注釈が必須
    let guess: i32 = "42".parse().expect("Not a number");
    println!("guess: {}", guess);
}
