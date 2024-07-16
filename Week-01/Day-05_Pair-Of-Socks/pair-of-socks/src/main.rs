use std::collections::HashMap;
fn main() {
    println!("{}",find_pair_of_socks("AABBCCCC"));
}

fn find_pair_of_socks(socks: &str) -> i32 {
    let mut socks_map = HashMap::new();
    let mut pair_of_socks = 0;

    for c in socks.chars() {
        let key = String::from(c);
        let count = socks_map.entry(key).or_insert(0);
        *count += 1;
    }

    for (key, value) in &socks_map {
        if *value > 1 {
            pair_of_socks +=(*value / 2)
        }
    }


    return pair_of_socks;
}
