// use std::vec;
use std::iter::AdditiveIterator;
// use std::collections::HashMap;
// use std::mem;
// use std::collections::TreeMap;

type Float = f64;
type Point = (Float, Float);

// type Map<K, V> = TreeMap<K, V>;

#[deriving(Show, PartialEq)]
struct Bounds((Float, Float), (Float, Float));

impl Bounds {
    fn contains(&self, (px, py): Point) -> bool {
        let &Bounds((x, y), (w, h)) = self;
        x < px && px <= (x + w) && y < py && py <= (y + h)
    }
}

struct Quadtree<T> where T: Sized {
    root: QuadtreeNode<T>
}

impl<T> Quadtree<T> {
    fn new(bounds: Bounds) -> Quadtree<T> {
        Quadtree { root: Leaf(bounds, Vec::new()) }
    }
    fn insert(&mut self, pt: Point, value: T) {
        self.root.insert(pt, value)
    }
}

// #[deriving(Show)]
enum QuadtreeNode<T> where T: Sized {
    Branch([Box<QuadtreeNode<T>>, ..4]),
    Leaf(Bounds, Vec<(Point, T)>)
}

impl<T> QuadtreeNode<T> {
    
    fn len(&self) -> uint {
        match *self {
            Branch(ref children) => children.iter().map(|child| child.len()).sum(),
            Leaf(_, ref data) => data.len()
        }
    }

    fn split(&mut self) {
        let mut children: [Box<QuadtreeNode<T>>, ..4] = unsafe {std::mem::uninitialized()};
        match self {
            &Leaf(Bounds((x0, y0), (w, h)), ref mut data) => {

                let make_bounds = |i: uint| {
                    let (dx, dy) = (w / 2.0, h / 2.0);
                    let x = if i % 2 == 0 { x0 } else { x0 + dx };
                    let y = if i >= 2 { y0 } else { y0 + dy };
                    Bounds((x, y), (w, h))
                };
                
                for (i, child) in children.iter_mut().enumerate() {
                    *child = box Leaf(make_bounds(i), Vec::new());
                };
            },
            _ => return
        }
        *self = Branch(children);
    }

    fn insert(&mut self, (x, y): Point, value: T) {
        match self {
            &Branch(ref mut children) => {
                // TODO
            },
            &Leaf(bounds, ref mut data) => data.push(((x, y), value))
        }
    }
}

fn main() {
    let bounds = Bounds((0.0, 0.0), (1.0, 1.0));
    let mut test: Quadtree<int> = Quadtree::new(bounds);
}
