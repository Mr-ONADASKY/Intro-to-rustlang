fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100])
}

fn main() {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i)
    }

    take(v);

    println!("Finished!")
}
