// use std::iter::AdditiveIterator;
// use std::ops::IndexMut;
// use std::num;
use std::mem::swap;
// use test::Bencher;

type Float = f64;
pub type Point = (Float, Float);

const MAX_BUCKET_CAPACITY: uint = 1;
const MAX_NODE_DEPTH: uint = 32;

pub struct Quadtree<T> where T: Sized {
    root: Node<T>,
    dimensions: (Float, Float),
}

impl<T> Quadtree<T> where T: Clone {
    pub fn new(center: Point, w: Float, h: Float) -> Quadtree<T> {
        Quadtree {
            root: Node {
                center: center,
                variant: Bucket(Vec::new())
            },
            dimensions: (w, h)
        }
    }
    
    pub fn len(&self) -> uint {
        let mut sum: uint = 0u;
        self.root.traverse(|node| match node.variant {
            Bucket(ref data) => sum += data.len(), _ => return
        });
        sum
    }
    
    pub fn push(&mut self, pt: Point, value: T) {
        let res = self.root.push(MAX_NODE_DEPTH, pt, value);
        match res {
            Some((node, depth_sub)) => {
                let depth = MAX_NODE_DEPTH - depth_sub;
                let (w, h) = self.dimensions;
                // let denom: Float = std::num::pow(2., depth);
                let denom = (2u << depth) as Float;
                unsafe {
                    (*node).split(w / denom, h / denom);
                }
            },
            None => return
        }
    }
}

type Child<T> = Box<Node<T>>;

// #[deriving(Show)]
enum NodeVariant<T> where T: Sized {
    Branch([Child<T>, ..4]),
    Bucket(Vec<(Point, T)>)
}

struct Node<T> {
    center: Point,
    variant: NodeVariant<T>
}

fn quadrant((x0, y0): Point, (x, y): Point) -> uint {
    match (x < x0, y < y0) {
        (true, true) => 0,
        (_, true)    => 1,
        (true, _)    => 2,
        _            => 3,
    }
}

impl<T> Node<T> where T: Clone {

    fn traverse(&self, f: |&Node<T>|) {
        f(self);
        match self.variant {
            Branch(ref children) => {
                for child in children.iter() {
                    child.traverse(|child| f(child));
                }
            },
            _ => return
        }
    }

    fn split(&mut self, w: Float, h: Float) {
        let (x0, y0) = self.center;
        let mut children: [Child<T>, ..4] = unsafe {::std::mem::uninitialized()};
        
        match self.variant {
            Bucket(ref mut data) => {

                let make_center = |i: uint| {
                    let (dx, dy) = (w / 2.0, h / 2.0);
                    let x = if i % 2 == 0 { x0 - dx } else { x0 + dx };
                    let y = if i < 2 { y0 - dy } else { y0 + dy };
                    (x, y)
                };
                
                let mut child_data: Vec<Vec<(Point, T)>> = Vec::from_elem(4, Vec::new());

                for val in data.iter() {
                    let &(pt, _) = val;
                    let q = quadrant(self.center, pt);
                    child_data.get_mut(q).push(val.clone());
                }

                for (i, child) in children.iter_mut().enumerate().rev() {
                    // println!("child_data.len = {}, i = {}", child_data.len(), i);
                    
                    let new = box Node {
                        center: make_center(i),
                        variant: Bucket(child_data.pop().unwrap())
                    };
                    unsafe { ::std::ptr::write(child, new); }
                }
            },
            _ => unreachable!()
        }
        swap(&mut self.variant, &mut Branch(children));
    }
    
    fn push(&mut self, depth: uint, pt: Point, value: T) -> Option<(*mut Node<T>, uint)> {
        // println!("push(depth={}, pt={}, _) (Node (center={}, len={}))", depth, pt, self.center, self.len());
            
        match self.variant {
            Branch(ref mut children) => {
                let q = quadrant(self.center, pt);
                children[q].push(depth - 1, pt, value)
            },
            Bucket(ref mut data) => {
                data.push((pt, value));
                if data.len() > MAX_BUCKET_CAPACITY && depth != 0 {
                    Some((self as *mut Node<T>, depth))
                } else {
                    None
                }
            }
        }
    }
}
