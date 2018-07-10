
Struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let p = Point { x: 12, y: 9 };
    let q = &p.x
    println!("{:?}", *q);
    //println!("{:?}", p < q);
}
