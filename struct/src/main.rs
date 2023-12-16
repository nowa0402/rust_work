fn main() {
    let user1 = User {
        email: String::from("sample@example.com"),
        username: String::from("Andy"),
        active: true,
        sign_in_count: 1,
    };

    // 構造体更新記法
    let user2 = User {
        email: String::from("ssss@example.com"),
        username: String::from("test"),
        ..user1
    };

    let color = Color(0, 0, 0);

    println!("user: {:?}", user1);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// フィールド省略記法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

// タプル構造体
struct Color(i32, i32, i32);
