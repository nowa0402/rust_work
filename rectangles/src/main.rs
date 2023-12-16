#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl(implementation) -> 実装ブロック
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数(::で呼び出せる)
    // クラスメソッドっぽい動き
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle::square(40);

    println!("square pixels: {}", rect1.area());
    println!("can hold: {}", rect1.can_hold(&rect2));
    println!("rect3: {:#?}", rect3);
}
