#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }
    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(8);
    ll.push_front(1);

    println!("ll = {:?}", ll);
}

// challenge insert_sorted
// find the first element it goes before and put it infront.
//impl <T:PartialOrd> LinkedList<T>{ ... }
