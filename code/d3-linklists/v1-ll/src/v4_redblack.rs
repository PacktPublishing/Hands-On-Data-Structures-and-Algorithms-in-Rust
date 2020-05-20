use std::fmt::Debug;
#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    //consider key value pair
    h: i8,
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
    pub fn rot_right(mut self) -> Box<Self> {
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }
    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left())
    }
    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right())
    }
}

impl<T: Debug> BinTree<T> {
    pub fn lfirst_print(&self, depth: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.lfirst_print(depth + 1);
            let mut spc = String::new();
            for _ in 0..depth {
                spc.push('.');
            }
            println!("{}\t   {}{:?}", bd.h, spc, bd.data);
            bd.right.lfirst_print(depth + 1);
        }
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    h: 1,
                    data,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };
        match dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

fn main() {
    let mut t = BinTree::new();
    for i in 0..100000 {
        t.add_sorted(i);
    }

    //println!("tree = {:?}",t);
    t.lfirst_print(0);

    println!("RandoTree");
    for _ in 0..1000 {
        t.add_sorted(rand::random::<u32>() % 5000);
    }

    //t.lfirst_print(0);
}
