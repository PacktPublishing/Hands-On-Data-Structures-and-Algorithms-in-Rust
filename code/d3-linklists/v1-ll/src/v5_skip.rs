use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Write};
use std::rc::Rc;
type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>> {
        //bigger than right
        if let Some(ref mut rt) = self.right {
            if dt > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(dt);
            }
        }
        //has lower children
        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        let dt = child.borrow().data.clone();
                        let nn = SkipNode {
                            right: self.right.take(),
                            data: dt,
                            down: Some(child),
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    }
                    false => None,
                },
                None => None,
            };
        }
        //bottom row dont move right
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }
}
impl<T: Debug + PartialOrd> SkipNode<T> {
    pub fn print_row<W: Write>(&self, w: &mut W) {
        write!(w, ",{:?}", self.data.borrow()).ok();
        if let Some(ref r) = self.right {
            r.borrow().print_row(w);
        }
    }
}

#[derive(Debug)]
/// zeroth element will be the bottom
pub struct SkipList<T: PartialOrd>(Vec<SkipNode<T>>);

impl<T: PartialOrd + Debug> SkipList<T> {
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    /// return whether the function parent should consider adding a ref
    pub fn insert(&mut self, data: T) {
        if self.0.len() == 0 {
            self.0.push(SkipNode::new(data));
            return;
        }
        //insert after fisrt place if fits after
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(ch) = self.0[i].insert(data) {
                    self.loop_up(ch, i + 1);
                }

                return;
            }
        }
        //insert as first element
        let mut nn = SkipNode::new(data);
        std::mem::swap(&mut nn, &mut self.0[0]);
        let res = rcc(nn);
        self.0[0].right = Some(res.clone());
        //TODO go up
        self.loop_up(res, 1);
    }

    pub fn loop_up(&mut self, ch: Rcc<SkipNode<T>>, n: usize) {
        if rand::random::<bool>() == true {
            return;
        }
        let dt = ch.borrow().data.clone();
        let mut nn = SkipNode {
            right: None,
            down: Some(ch),
            data: dt,
        };
        if n >= self.0.len() {
            self.0.push(nn);
            return;
        }

        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rcc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n + 1);
    }
}

impl<T: Debug + PartialOrd> fmt::Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 {
            write!(f, "SkipList<Empty>")?;
            return Ok(());
        }
        for sn in &self.0 {
            write!(f, "\n")?;
            sn.print_row(f);
        }
        Ok(())
    }
}

fn main() {
    let mut sl = SkipList::new();
    sl.insert(4);
    sl.insert(2);
    sl.insert(6);
    sl.insert(5);
    sl.insert(7);
    sl.insert(11);
    sl.insert(7);
    sl.insert(41);
    sl.insert(12);
    sl.insert(31);
    sl.insert(9);

    println!("{}", sl);
}
