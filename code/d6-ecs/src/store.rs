use crate::gen::GenData;
//possibly hashstore tree store etc,
pub trait EcsStore<T> {
    fn add(&mut self, g: GenData, t: T);
    fn get(&self, g: GenData) -> Option<&T>;
    fn get_mut(&mut self, g: GenData) -> Option<&mut T>;
    fn drop(&mut self, g: GenData);
    fn for_each<F: FnMut(GenData, &T)>(&self, f: F);
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, f: F);
}

pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>,
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        VecStore { items: Vec::new() }
    }
}

impl<T> EcsStore<T> for VecStore<T> {
    fn add(&mut self, g: GenData, t: T) {
        while g.pos >= self.items.len() {
            self.items.push(None);
        }
        self.items[g.pos] = Some((g.gen, t));
    }

    fn get(&self, g: GenData) -> Option<&T> {
        if let Some(Some((ig, d))) = self.items.get(g.pos) {
            if *ig == g.gen {
                return Some(d);
            }
        }
        None
    }

    fn get_mut(&mut self, g: GenData) -> Option<&mut T> {
        if let Some(Some((ig, d))) = self.items.get_mut(g.pos) {
            if *ig == g.gen {
                return Some(d);
            }
        }
        None
    }
    fn drop(&mut self, g: GenData) {
        if let Some(Some((ig, _))) = self.items.get(g.pos) {
            if *ig == g.gen {
                self.items[g.pos] = None;
            }
        }
    }

    fn for_each<F: FnMut(GenData, &T)>(&self, mut f: F) {
        for (n, x) in self.items.iter().enumerate() {
            if let Some((g, d)) = x {
                f(GenData { gen: *g, pos: n }, d)
            }
        }
    }

    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f: F) {
        for (n, x) in self.items.iter_mut().enumerate() {
            if let Some((g, d)) = x {
                f(GenData { gen: *g, pos: n }, d)
            }
        }
    }
}

/*

pub struct VSIter<'a,T>{
    v:&'a Vec<Option<(u64,T)>>,
    p:usize,
}

impl<'a,T> Iterator for VSIter<'a,T>{
    type Item = (GenData,&'a T);
    fn next(&mut self)->Option<Self::Item>{
        self.p +=1 ;
        self.v.get(self.p-1).map(|(rg,rdat)|(GenData{pos:self.p-1,gen:*rg},rdat))

        /*if self.p < self.v.len(){
            let (ref rg,rdata) =  self.v.get(self.p).clone()?;
            self.p +=1;
            return Some((GenData{pos:self.p-1, gen:*rg},rdata));
        }
        None*/
    }
}

impl<'a, T> IntoIterator for &'a VecStore<T>{
    type Item = (GenData,&'a T);
    type IntoIter = VSIter<'a,T>;
    fn into_iter(self)->VSIter<'a,T>{
        VSIter{
            p :0,
            v:&self.items,
        }
    }
}
*/
