use std::cmp::{Ordering, Reverse};

struct Poly<const N: usize> {
    coef: [f64; N]
}

fn main() {
    print!("Hello, world!");
    //println!("Is it zero? {}", 1. / f64::INFINITY);
    //std::f64::consts::PI
    //std::f32::consts::PI
    //let mut v = vec![0; 10];
    //v.push(2);
    //let mut f = "first".to_string();
    //f = "ddd".to_string();
    //let mut v = vec![0; 10];
    //std::mem::rep
    //"test".eq("test");
    //"test".to_string().eq("test");
    //let ply = Poly<{5+1}>()
    //matches!( 0 >= 1, Some(Ordering::Less | Ordering::Equal) );
    //std::cmp::Reverse
    //let mut v = vec![1, 2, 3, 4, 5, 6];
    //v.sort_by_key(|&num| (num > 3, Reverse(num)));
    //println!("{:?}", v);
    let v = [1,2,3,4];
    assert_eq!(v.to_vec(), vec![1,2,3,4]);
    //assert_eq!(vec![v], vec![1,2,3,4]);
}