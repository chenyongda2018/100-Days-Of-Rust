
fn main() {
    test_struct();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}

// fn test_String() {
//     let mut s = String::from("Lu yan");
//     s.push_str(" 666");
//     println!("{}", s);
// }
//
// fn test_String2() {
//     let s1 = String::from("hell");
//     let s2 = s1;
//     println!("{}",s2);
// }


// fn test_string3() {
//     let s = String::from("hello");
//     takes_ownership(s);
//     println!("在move进函数后继续使用s,{}",s);
// }
//
// fn takes_ownership(some_string:String) {
//     println!("{}",some_string);
// }


// fn test_tuple() {
//     let x = (1,"h",1.1);
//     let (q,w,e) = x;
//     println!("{},{}",x.0,q);
// }

fn test_struct() {
    let user = User {
        active: true,
        username: String::from("zhangsan"),
        email: String::from("123@qq.com"),
        sign_in_account:123
    };
    println!("{}",user.active)
}