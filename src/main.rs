#![feature(extend_one)]

fn main() {
    let mut l = vec![1, 2, 3];
    let l2 = vec![4, 5, 6];

    l.extend_one(4);

    println!("{:?}", l);
}
