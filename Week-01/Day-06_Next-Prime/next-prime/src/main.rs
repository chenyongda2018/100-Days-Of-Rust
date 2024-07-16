fn main() {
    let num = 10;
    println!("The {} next prime is {}",num,find_next_prime(num));
}


fn find_next_prime(num: i32) -> i32 {
    let mut i = num;
    while !isPrime(i) {
        i += 1;
    }
    return i;
}


fn isPrime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}
