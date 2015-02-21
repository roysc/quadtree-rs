#![feature(slicing_syntax)]
// #![feature(phase)]

// #[phase(plugin, link)] extern crate log;
use std::time;

mod quadtree;

// TODO make time work...
fn now() -> u64 { time::precise_time_ns() }

macro_rules! benchmark(
    ($what: expr) => {
        {        
            let start_time = now();
            let ret = $what;
            let end_time = now();
            (ret, end_time - start_time)
        }
    }
);

fn main() {
    use quadtree::{Quadtree, Point};
    use std::rand::distributions::{IndependentSample, Range};
    use std::rand::Rng;

    let args = std::os::args();
    let n = match &*args {
        [_, ref a, ..] => match a[].parse::<u32>() {
            Some(n) => n,
            _ => {
                println!("enter a positive integer");
                return
            }
        },
        _ => 1000
    };
    
    let mut tree: Quadtree<i32> = Quadtree::new((0.0, 0.0), 1.0, 1.0);

    let mut rng = std::rand::thread_rng();
    let dist = Range::new(-0.5, 0.5);
    
    let points = (1..n).map(|_| -> (Point, i32) {
        let p = (dist.ind_sample(&mut rng), dist.ind_sample(&mut rng));
        (p, rng.gen())
    }).collect::<Vec<(Point, i32)>>();

    let (_, t_insert) = benchmark!(for e in points.into_iter() { tree.push(e); });
    
    println!("size = {}", tree.len());
    println!("nodes = {}", tree.node_count());
    println!("time = {} ms", t_insert as f64 / 1e6);
}
