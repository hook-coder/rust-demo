mod main3;
mod main4;
// 普通结构
struct User {
    active: bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.username = String::from("hook");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    build_user(user2.email,user2.username);

    main3::demo();
    main3::demo2();
    main3::demo3();
    
    main4::demo();
    
    main3::demo4();
}


fn build_user(email:String, username:String) -> User {
    User {
        active:true,
        email,
        username,
        sign_in_count:1,
    }
}