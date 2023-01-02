// 结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// 元组结构体
struct Color(i32, i32, i32);
fn main() {
    let user1 = build_user(String::from("email"),String::from( "username"));
    println!("{}", user1.email);
    // let user2 = User {
    //     active: user1.active,
    //     username: String::from("user2"),
    //     email: user1.email,
    //     sign_in_count: user1.sign_in_count
    // };
    // println!("{}",user2.username);
    let user3 = User {
        username: String::from("user3"),
        ..user1
    };
    let black = Color(0,0,0);
}
fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}