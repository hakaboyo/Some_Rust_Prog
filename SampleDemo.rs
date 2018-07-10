
Struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let p = Point { x: 12, y: 9 };
    let q = &p.x
    println!("{:?}", *q);
    //println!("{:?}", p < q);
    println!("{}", dot(&p, &q));
}

fn dot (p: &Point, q: &Point) -> u64 {
    p.x * q.x + p.y * q.y
}
