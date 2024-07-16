fn main() {
    test_string3();
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


fn test_string3() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("在move进函数后继续使用s,{}",s);
}

fn takes_ownership(some_string:String) {
    println!("{}",some_string);
}
