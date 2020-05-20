use std::fmt::Debug;
#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);


#[derive(Debug)]
pub struct BinData<T> {
    //consider key value pair
    data:T,
    left:BinTree<T>,
    right:BinTree<T>,
}

impl<T> BinData<T>{
    pub fn rot_left(mut self)->Box<Self>{
        let mut res = match self.right.0.take(){
            Some(res)=>res,
            None=>return Box::new(self),
        };
        self.right = BinTree(res.left.0.take());
        res.left = BinTree(Some(Box::new(self)));
        res
    }
    pub fn rot_right(mut self)->Box<Self>{
        let mut res = match self.left.0.take(){
            Some(res)=>res,
            None=>return Box::new(self),
        };
        self.left = BinTree(res.right.0.take());
        res.right = BinTree(Some(Box::new(self)));
        res
    }
}

impl<T> BinTree<T>{
    pub fn new()->Self{
        BinTree(None)
    }

    pub fn rot_left(&mut self){
        self.0 = self.0.take().map(|v|v.rot_left())
    }
    pub fn rot_right(&mut self){
        self.0 = self.0.take().map(|v|v.rot_right())
    }
}

impl<T:Debug> BinTree<T>{
    pub fn lfirst_print(&self,depth:i32){
        if let Some(ref bd)=self.0{
                bd.left.lfirst_print(depth+1);
                let mut spc = String::new();
                for _ in 0..depth{
                    spc.push(' ');
                }
                println!("{}{:?}",spc,bd.data);
                bd.right.lfirst_print(depth+1);
        }
    }
}

impl<T:PartialOrd> BinTree<T>{
    pub fn add_sorted(&mut self ,data:T){
        match self.0 {
            Some(ref mut bd)=>{
                if data < bd.data{
                    bd.left.add_sorted(data);
                }else {
                    bd.right.add_sorted(data);
                }

            }
            None=>self.0 = Some(Box::new(BinData{data,left:BinTree::new(),right:BinTree::new()})),
        }

    }

}



fn main(){
    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(6);
    t.add_sorted(1);
    t.add_sorted(9);
    t.add_sorted(8);
    t.add_sorted(13);
    t.add_sorted(45);
    t.add_sorted(0);
    t.add_sorted(20);
   
    //println!("tree = {:?}",t);
    t.lfirst_print(0);
    println!(" -- rotleft -- ");
    t.rot_left();
    t.lfirst_print(0);
}


