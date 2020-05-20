use crate::error::BlobError;
use serde::{Deserialize, Serialize};

pub fn read_u64<R: std::io::Read>(r: &mut R) -> Result<u64, BlobError> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(bincode::deserialize(&buf[..])?)
}

pub fn write_u64<W: std::io::Write>(w: &mut W, dat: u64) -> anyhow::Result<()> {
    let ec = bincode::serialize(&dat)?;
    assert_eq!(ec.len(), 8);
    Ok(w.write_all(&ec)?)
}

#[derive(Debug, PartialEq)]
pub struct Blob {
    k: Vec<u8>,
    v: Vec<u8>,
}

impl Blob {
    pub fn from<K: Serialize, V: Serialize>(k: &K, v: &V) -> Result<Blob, bincode::Error> {
        Ok(Blob {
            k: bincode::serialize(k)?,
            v: bincode::serialize(v)?,
        })
    }

    pub fn out<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        //no way serializing a usize can fail
        let klen = bincode::serialize(&self.k.len()).unwrap();
        let vlen = bincode::serialize(&self.v.len()).unwrap();

        w.write_all(&klen)?;
        w.write_all(&vlen)?;
        w.write_all(&self.k)?;
        w.write_all(&self.v)?;
        Ok(())
    }

    pub fn read<R: std::io::Read>(r: &mut R) -> anyhow::Result<Blob> {
        //if klen == 0, that is free space

        let klen = read_u64(r)? as usize;
        let vlen = read_u64(r)? as usize;

        let mut k = vec![0u8; klen];
        let mut v = vec![0u8; vlen];
        r.read_exact(&mut k)?;
        r.read_exact(&mut v)?;
        Ok(Blob { k, v })
    }

    pub fn get_v<'a, V: Deserialize<'a>>(&'a self) -> Result<V, BlobError> {
        Ok(bincode::deserialize(&self.v[..])?)
    }

    pub fn len(&self) -> u64 {
        (16 + self.k.len() + self.v.len()) as u64
    }

    //Note added [lib] name="d5_hashmap" and made hash pub
    pub fn k_hash(&self, seed: u64) -> u64 {
        d5_hashmap::hash(seed, &self.k)
    }

    pub fn key_match(&self, rhs: &Self) -> bool {
        self.k == rhs.k
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::*;
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Point<T> {
        x: T,
        y: T,
    }

    #[test]
    fn test_read_write_string() {
        let k: i32 = 87;
        let v = "hello world";
        let blob = Blob::from(&k, &v).unwrap();
        {
            let mut fout = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open("test_data/t_rblob.dat")
                .unwrap();
            blob.out(&mut fout).unwrap();
        }

        let mut fin = std::fs::File::open("test_data/t_rblob.dat").unwrap();
        let b2 = Blob::read(&mut fin).unwrap();
        let v2: String = b2.get_v().unwrap();
        assert_eq!(&v2, "hello world");

        //It's just Bytes

        let p: Point<i32> = b2.get_v().unwrap();
        assert_eq!(p, Point { x: 11, y: 0 });
    }

    #[test]
    pub fn test_ser64() {
        let ndat = bincode::serialize(&0u64).unwrap();
        assert_eq!(ndat.len(), 8);
    }
}
