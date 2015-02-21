// use std::iter::AdditiveIterator;
// use std::ops::IndexMut;

// use NodeVariant::{Branch, Bucket};

const MAX_BUCKET_CAPACITY: usize = 16;
const MAX_NODE_DEPTH: usize = 32;

pub type Point = (f64, f64);

pub struct Quadtree<T> where T: Sized {
    root: Node<T>,
    dimensions: (f64, f64),
}

type Child<T> = Box<Node<T>>;

// #[deriving(Show)]
enum NodeVariant<T> where T: Sized {
    Branch([Child<T>; 4]),
    Bucket(Vec<(Point, T)>)
}

struct Node<T> {
    center: Point,
    variant: NodeVariant<T>
}

impl<T> Quadtree<T> {
    pub fn new(center: Point, w: f64, h: f64) -> Quadtree<T> {
        Quadtree {
            root: Node {
                center: center,
                variant: NodeVariant::Bucket(Vec::new())
            },
            dimensions: (w, h)
        }
    }
    
    pub fn len(&self) -> usize {
        let mut sum: usize = 0;
        self.root.traverse(
            &mut |node| match node.variant {
                NodeVariant::Bucket(ref data) => sum += data.len(),
                _ => return
            });
        sum
    }

    pub fn node_count(&self) -> usize {
        let mut sum: usize = 0;
        self.root.traverse(&mut |_| sum += 1);
        sum
    }
    
    pub fn push(&mut self, e: (Point, T)) {
        let res = self.root.push(MAX_NODE_DEPTH, e);
        match res {
            Some((node, depth_sub)) => {
                let (w, h) = self.dimensions;
                let depth = MAX_NODE_DEPTH - depth_sub + 1;
                // let denom: f64 = ::std::num::pow(2., depth);
                let denom = (2 << depth) as f64;
                unsafe {
                    (*node).split(w / denom, h / denom);
                }
            },
            None => return
        }
    }
}

fn quadrant((x0, y0): Point, (x, y): Point) -> usize {
    match (x < x0, y < y0) {
        (true, true) => 0,
        (_, true)    => 1,
        (true, _)    => 2,
        _            => 3,
    }
}

impl<T> Node<T> {

    fn traverse(&self, f: &mut FnMut(&Node<T>) -> ()) {
        f(self);
        match self.variant {
            NodeVariant::Branch(ref children) => {
                for child in children.iter() {
                    child.traverse(&mut |child| f(child));
                }
            },
            _ => return
        }
    }

    fn split(&mut self, dx: f64, dy: f64) {
        use std::mem::swap;
        
        let (x0, y0) = self.center;
        // let mut children: [Child<T>, ..4] = unsafe {::std::mem::uninitialized()};
        let mut children: [Child<T>; 4] = unsafe {::std::mem::uninitialized()};
        
        match self.variant {
            NodeVariant::Bucket(ref mut data) => {

                let make_center = |&: quadrant: usize| {
                    let x = if quadrant % 2 == 0 { x0 - dx } else { x0 + dx };
                    let y = if quadrant < 2 { y0 - dy } else { y0 + dy };
                    (x, y)
                };
                
                // let mut child_data: Vec<Vec<(Point, T)>> = vec![Vec::new(); 4];
                let mut child_data: Vec<Vec<(Point, T)>> = Vec::new();
                // child_data.resize(4, Vec::new());
                // let mut child_data: Vec<Vec<(Point, T)>> = Vec::from_elem(4, Vec::new());
                for _ in 1..4 { child_data.push(Vec::new()); }
                let mut data_swap = Vec::new();
                swap(data, &mut data_swap);

                for (pt, val) in data_swap.into_iter() {
                    let q = quadrant(self.center, pt);
                    child_data[q].push((pt, val));
                }

                for (i, child) in children.iter_mut().enumerate().rev() {
                    // println!("child_data.len = {}, i = {}", child_data.len(), i);
                    
                    let new = Box::new(Node {
                        center: make_center(i),
                        variant: NodeVariant::Bucket(child_data.pop().unwrap())
                    });
                    unsafe { ::std::ptr::write(child, new); }
                }
            },
            _ => unreachable!()
        }
        self.variant = NodeVariant::Branch(children);
    }
    
    fn push(&mut self, depth: usize, (pt, value): (Point, T)) -> Option<(*mut Node<T>, usize)> {
        // println!("push(depth={}, pt={}, _) (Node (center={}))", depth, pt, self.center);
            
        match self.variant {
            NodeVariant::Branch(ref mut children) => {
                let q = quadrant(self.center, pt);
                children[q].push(depth - 1, (pt, value))
            },
            NodeVariant::Bucket(ref mut data) => {
                data.push((pt, value));
                if data.len() == MAX_BUCKET_CAPACITY && depth != 0 {
                    Some((self as *mut Node<T>, depth))
                } else {
                    None
                }
            }
        }
    }
}
