fn main() {
    let s1 = vec![0,0,0,1,2,3];
    let s2 = vec![2,5,6];
    let result = merge(&s1,&s2);
    println!("{:?}",result);
}

fn merge(s1: &[i32],s2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(s1.len() + s2.len());
    result.extend_from_slice(s1);
    result.extend_from_slice(s2);
    result.sort();
    result
}
