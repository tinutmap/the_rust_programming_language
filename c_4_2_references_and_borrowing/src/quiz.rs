fn get_first(vr: &Vec<i32>) -> i32 {
    vr[0]
}

pub fn main() {
    let v = vec![0, 1, 2];
    let n = get_first(&v);
    println!("{} {}", n, v[1]);
}
