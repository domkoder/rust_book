struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
// Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32,i32);



fn build_user(username: String, email:String, )-> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn main() {
    let user1 = build_user(
        String::from("testnet"), 
        String::from("test@gmail.com") 
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}