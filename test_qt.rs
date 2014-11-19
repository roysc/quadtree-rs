#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(phase)]

// #[phase(plugin, link)] extern crate log;
extern crate time;

mod quadtree;

macro_rules! benchmark(
    ($what: expr) => { {        
        let start_time = time::precise_time_ns();
        let ret = $what;
        let end_time = time::precise_time_ns();
        (ret, end_time - start_time)
    } }
)

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
        _ => 1000
    };
    
    let mut tree: Quadtree<int> = Quadtree::new((0.0, 0.0), 1.0, 1.0);

    let mut rng = std::rand::task_rng();
    let dist = Range::new(-0.5, 0.5);

    let points = Vec::<(Point, int)>::from_fn(n, |_| {
        let p = (dist.ind_sample(&mut rng), dist.ind_sample(&mut rng));
        (p, rng.gen())
    });

    let (_, t_insert) = benchmark!(for e in points.into_iter() { tree.push(e); });
    
    println!("size = {}", tree.len());
    println!("nodes = {}", tree.node_count());
    println!("time = {} ms", t_insert as f64 / 1e6);
}
