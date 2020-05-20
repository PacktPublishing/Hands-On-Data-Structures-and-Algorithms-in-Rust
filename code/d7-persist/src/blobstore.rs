use crate::error::BlobError;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use crate::blob::{read_u64, write_u64, Blob};

const CONT_SIZE: u64 = 32;

pub struct BlobStore {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
    elems: u64,
}

//Use a pair of files and achieve

impl BlobStore {
    pub fn new(fname: &str, block_size: u64, nblocks: u64) -> anyhow::Result<Self> {
        let hseed = rand::random::<u64>();
        //create file
        let mut ff = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(fname)?;
        let f = &mut ff;
        f.set_len(block_size * nblocks + CONT_SIZE)?;
        f.seek(SeekFrom::Start(0))?;
        write_u64(f, hseed)?;
        write_u64(f, block_size)?;
        write_u64(f, nblocks)?;
        write_u64(f, 0)?;

        for x in 0..nblocks {
            f.seek(SeekFrom::Start(CONT_SIZE + x * block_size))?;
            //klen == 0 means empty section
            write_u64(f, 0)?;
            write_u64(f, block_size - 16)?;
        }
        Ok(BlobStore {
            hseed,
            file: ff,
            block_size,
            nblocks,
            elems: 0,
        })
    }

    pub fn open(fname: &str) -> anyhow::Result<Self> {
        let mut ff = OpenOptions::new().write(true).read(true).open(fname)?;
        ff.seek(SeekFrom::Start(0))?;
        println!("pre_nums");
        let f = &mut ff;
        let hseed = read_u64(f)?;
        let block_size = read_u64(f)?;
        let nblocks = read_u64(f)?;
        let elems = read_u64(f)?;
        println!("post_nums");
        Ok(BlobStore {
            hseed,
            file: ff,
            block_size,
            nblocks,
            elems,
        })
    }

    pub fn n_elems(&self) -> u64 {
        self.elems
    }

    pub fn new_or_open(fname: &str, bsize: u64, nblocks: u64) -> anyhow::Result<Self> {
        Self::new(fname, bsize, nblocks).or_else(|_| Self::open(fname))
    }

    //Does not remove prior keys
    pub fn insert_only<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<(), BlobError> {
        let blob = Blob::from(&k, &v)?;
        if blob.len() > self.block_size {
            return Err(BlobError::TooBig(blob.len()));
        }
        let bucket = blob.k_hash(self.hseed) % self.nblocks;

        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(CONT_SIZE + self.block_size * bucket))?;
        println!("pos = {}", pos);
        //start each loop in at front of block elem
        loop {
            if pos >= CONT_SIZE + self.block_size * (bucket + 1) {
                //Solutions Maybe just stick overflows on the back of the file??
                //Try Defrag
                return Err(BlobError::NoRoom);
            }
            //look for empty spor big enough
            println!("insert get lens");
            let klen = read_u64(f)?;
            let vlen = read_u64(f)?;
            if klen == 0 && blob.len() < vlen {
                println!("insert go back to add here");
                f.seek(SeekFrom::Start(pos))?;
                println!("insert blob add");
                blob.out(f)?;
                //add back leftoverdata
                println!("insert back add");
                write_u64(f, 0)?;
                write_u64(f, (vlen - blob.len()) - 16)?;
                return Ok(());
            }

            println!("insert add here");
            pos = f.seek(SeekFrom::Start(pos + 16 + klen + vlen))?;
        }
    }

    pub fn get<K: Serialize>(&mut self, k: &K) -> Result<Blob, BlobError> {
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;

        let mut pos = f.seek(SeekFrom::Start(CONT_SIZE + self.block_size * bucket))?;
        //start each loop in at front of block elem
        loop {
            if pos >= CONT_SIZE + self.block_size * (bucket + 1) {
                return Err(BlobError::NotFound);
            }
            let b = Blob::read(f)?;
            if b.key_match(&s_blob) {
                return Ok(b);
            }
            pos += b.len();
        }
    }

    pub fn remove<K: Serialize>(&mut self, k: &K) -> Result<(), BlobError> {
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;

        let mut pos = f.seek(SeekFrom::Start(CONT_SIZE + self.block_size * bucket))?;
        //start each loop in at front of block elem
        let b_end = CONT_SIZE + self.block_size * (bucket + 1);
        loop {
            if pos >= b_end {
                return Ok(());
            }
            let b = Blob::read(f)?;
            if b.key_match(&s_blob) {
                let l = b.len();
                //check if next block is empty, then we can join them

                if pos + l < b_end {
                    if read_u64(f)? == 0 {
                        let nlen = read_u64(f)?;
                        f.seek(SeekFrom::Start(pos))?;
                        write_u64(f, 0)?;
                        write_u64(f, l + nlen + 16)?;
                        return Ok(());
                    }
                }
                f.seek(SeekFrom::Start(pos))?;
                write_u64(f, 0)?;
                write_u64(f, l - 16)?;

                return Ok(());
            }
            pos += b.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_get_gets_thing() {
        std::fs::remove_file("test_data/bs_1").ok();

        let mut bs = BlobStore::new("test_data/bs_1", 1000, 10).unwrap();
        bs.insert_only(55, "hello people").unwrap();
        bs.insert_only("green", "Another really long data thing")
            .unwrap();
        bs.insert_only("sandwich", vec!["young", "old", "not_sure"])
            .unwrap();

        //let blob = bs.get(&55).unwrap();
        assert_eq!(
            bs.get(&55).unwrap().get_v::<String>().unwrap(),
            "hello people".to_string()
        );
        assert_eq!(
            bs.get(&"green").unwrap().get_v::<String>().unwrap(),
            "Another really long data thing".to_string()
        );
        let vback: Vec<String> = bs.get(&"sandwich").unwrap().get_v().unwrap();
        assert_eq!(
            vback,
            vec![
                "young".to_string(),
                "old".to_string(),
                "not_sure".to_string()
            ]
        );

        bs.remove(&"green").unwrap();

        assert!(bs.get(&"green").is_err());
        assert!(bs.get(&55).is_ok());
    }

    #[test]
    pub fn test_reread() {
        std::fs::remove_file("test_data/bs_reread").ok();
        let bs = BlobStore::new("test_data/bs_reread", 1000, 10).unwrap();
        drop(bs);
        let b2 = BlobStore::open("test_data/bs_reread").unwrap();
        assert_eq!(b2.block_size, 1000);
    }
}
