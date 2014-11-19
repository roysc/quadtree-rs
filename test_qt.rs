#![feature(slicing_syntax)]
#![feature(phase)]

// #[phase(plugin, link)] extern crate log;
extern crate time;

mod quadtree;
    
// fn bench(f: ||) -> u64 {
//     let start_time = time::precise_time_ns();
//     f();
//     let end_time = time::precise_time_ns();
//     return end_time - start_time;
// }

fn main() {
    use quadtree::{Quadtree, Point};
    use std::rand::distributions::{IndependentSample, Range};
    use std::rand::Rng;

    let args = std::os::args();
    let n = match args[] {
        [_, ref a, ..] => match from_str::<uint>(a[]) {
            Some(n) => n, _ => {
                println!("enter a positive integer");
                return
            }
        },
        _ => 10
    };
    
    let mut test: Quadtree<int> = Quadtree::new((0.0, 0.0), 1.0, 1.0);

    let mut rng = std::rand::task_rng();
    let dist = Range::new(-0.5, 0.5);

    let points = Vec::<(Point, int)>::from_fn(n, |_| {
        let p = (dist.ind_sample(&mut rng), dist.ind_sample(&mut rng));
        (p, rng.gen())
    });

    let start_time = time::precise_time_ns();
    for e in points.into_iter() { test.push(e); }
    let end_time = time::precise_time_ns();
    
    println!("size = {}", test.len());
    println!("nodes = {}", test.node_count());
    println!("time = {} ms", (end_time - start_time) as f64 / 1e6);
}
