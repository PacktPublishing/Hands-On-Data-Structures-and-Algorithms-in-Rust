use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

//edgelist
pub struct EdgeListGraph<E, ID> {
    v: Vec<(E, ID, ID)>,
}

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

// Pointer based
pub struct RccGraph<T> {
    nodes: Vec<Rcc<RccNode<T>>>,
}

pub struct RccNode<T> {
    data: T,
    edges: Vec<Weak<RefCell<RccNode<T>>>>,
}

// MapBased
pub struct MapGraph<T, E, ID: Hash> {
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}
//MapPointer

pub struct MapPGraph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

pub struct MapRcGraph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<Rc<(E, ID, ID)>>)>,
}
