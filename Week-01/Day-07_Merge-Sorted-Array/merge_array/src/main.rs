fn main() {
    let s1 = vec![0,0,0,1,2,3];
    let s2 = vec![2,5,6];
    let result = merge_array(s1,s2);
    println!("{:?}",result);
}


fn merge_array(mut s1:Vec<i32>,mut s2:Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(s1.len()+s2.len());
    s1.sort_unstable();
    s2.sort_unstable();

    let mut i = 0;
    let mut j = 0;
    while i < s1.len() && j < s2.len() {
        if s1[i] < s2[j] {
            result.push(s1[i]);
            i+=1;
        } else {
            result.push(s2[j]);
            j+=1;
        }
    }

    if i < s1.len() {
        result.append(&mut s1[i..s1.len()].to_vec());
    }

    if j < s2.len() {
        result.append(&mut s2[j..s2.len()].to_vec());
    }

    return result;
}
