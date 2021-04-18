fn main() {
    proconio::input!{
        x: i32
    }
    println!("{}", x + x.pow(2) + x.pow(3));
}
