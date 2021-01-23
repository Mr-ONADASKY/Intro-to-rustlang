fn main() {
    let t = (1, 'a', false);
    let f = (2, (1, 'a', false));
    println!("{} {} {}", t.0, t.1, t.2);
    println!("{:#?}", f);
}
