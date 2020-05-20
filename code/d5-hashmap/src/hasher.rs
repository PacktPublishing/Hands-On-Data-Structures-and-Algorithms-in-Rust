use std::hash::{Hash, Hasher};

pub struct MHash {
    prev: u8,
    n: u128,
}

impl Hasher for MHash {
    fn write(&mut self, dt: &[u8]) {
        for d in dt {
            self.n = (((self.n + 11) * (*d as u128 + 13)) + ((d ^ self.prev) as u128))
                % (std::u64::MAX as u128);
            self.prev = *d;
        }
    }

    fn finish(&self) -> u64 {
        self.n as u64
    }
}

pub fn hash<T: Hash>(seed: u64, t: T) -> u64 {
    let mut h = MHash { n: 0, prev: 0 };
    h.write_u64(seed);
    t.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_hasher() {
        let n = hash(55, "cat");
        //println!("cathash = {}", n);
        assert_eq!(n, hash(55, "cat")); // fail by compare with wrong str

        assert!(hash(55, "abc") != hash(55, "cba"));
    }

    #[test]
    pub fn test_numbers() {
        let mut prev = 0;
        for x in 0..10000 {
            let curr = hash(55, x);
            println!("curr = {}", curr);
            assert!(curr != prev);
            prev = curr;
        }
    }
}
