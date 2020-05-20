pub use hasher::hash;
use std::borrow::Borrow;
use std::hash::Hash;

mod hasher;

pub const BSIZE: usize = 8;
pub const BGROW: usize = 8;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V>
where
    K: Borrow<K>,
{
    fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }
    //requires a precheck elsewhere that the item is not in the list
    fn push(&mut self, k: K, v: V) -> usize {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        self.buckets[h].push((k, v));
        self.len += 1;
        self.buckets[h].len()
    }

    fn get<KB>(&self, k: &KB) -> Option<&V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h] {
            if k == ik.borrow() {
                return Some(iv);
            }
        }
        None
    }
    fn get_mut<KB>(&mut self, k: &KB) -> Option<&mut V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, k) as usize) % self.buckets.len();
        for (ik, iv) in &mut self.buckets[h] {
            if k == (ik as &K).borrow() {
                return Some(iv);
            }
        }
        None
    }

    fn bucket(&mut self, n: usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[n]);
        self.len -= res.len();
        Some(res)
    }

    fn set_buckets(&mut self, n: usize) {
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new())
        }
    }
}

#[derive(Debug)]
pub struct HMap<K, V> {
    n_moved: usize, //num moved to next bucket
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}

impl<K: Hash + Eq, V> HMap<K, V>
where
    K: Borrow<K>,
{
    pub fn new() -> Self {
        HMap {
            n_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(iv) = self.main.get_mut(&k) {
            *iv = v;
            return;
        }
        if let Some(iv) = self.grow.get_mut(&k) {
            *iv = v;
            return;
        }

        if self.n_moved > 0 {
            self.grow.push(k, v);
            self.move_bucket();
            return;
        }

        if self.main.push(k, v) > BSIZE / 2 {
            self.move_bucket();
        }
    }

    pub fn get_mut<KR>(&mut self, kr: &KR) -> Option<&mut V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        if let Some(b) = self.main.get_mut(kr) {
            return Some(b);
        }
        self.grow.get_mut(kr)
    }

    pub fn get<KR>(&self, kr: &KR) -> Option<&V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get(kr).or_else(|| self.grow.get(kr))
    }

    pub fn len(&self) -> usize {
        self.main.len + self.grow.len
    }

    fn move_bucket(&mut self) {
        if self.n_moved == 0 {
            self.grow.set_buckets(self.main.buckets.len() + BGROW);
        }
        if let Some(b) = self.main.bucket(self.n_moved) {
            for (k, v) in b {
                self.grow.push(k, v);
            }
            self.n_moved += 1;
            return;
        }
        std::mem::swap(&mut self.main, &mut self.grow);
        self.n_moved = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_right_value() {
        let mut hm = HMap::new();
        hm.insert("james".to_string(), 7);
        hm.insert("sam".to_string(), 5);
        hm.insert("hal".to_string(), 4);
        hm.insert("bob".to_string(), 11);
        hm.insert("greg".to_string(), 55);
        hm.insert("irene".to_string(), 9);
        hm.insert("dave".to_string(), 12);
        hm.insert("harper".to_string(), 14);
        hm.insert("river".to_string(), 100);
        hm.insert("pete".to_string(), 54);

        hm.insert("andy".to_string(), 7);
        hm.insert("arragog".to_string(), 5);
        hm.insert("thorin".to_string(), 4);
        hm.insert("geralt".to_string(), 11);
        hm.insert("andrex".to_string(), 55);
        hm.insert("gimly".to_string(), 9);
        hm.insert("roger".to_string(), 12);
        hm.insert("tamaran".to_string(), 14);
        hm.insert("owen".to_string(), 100);
        hm.insert("garfield".to_string(), 54);

        assert_eq!(hm.get_mut("james"), Some(&mut 7));
        assert_eq!(hm.get_mut("garfield"), Some(&mut 54));
        assert_eq!(hm.get_mut("geralt"), Some(&mut 11));
        assert_eq!(hm.get_mut("pete"), Some(&mut 54));
        assert_eq!(hm.get_mut("river"), Some(&mut 100));

        assert_eq!(hm.len(), 20);

        println!("hm = {:?}", hm);
        panic!();
    }
}
