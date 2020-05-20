mod b_rand;

fn main() {
    let mut v = vec![1, 34, 6, 12, 8, 100, 320, 66, 90, 2000, 45, 65, 120];

    println!("v = {:?}", v);

    //    insert_sort(&mut v);
    //bubble_sort(&mut v);
    //let v = merge_sort(v);
    println!("sorted v = {:?}", v);

    let big_gen = b_rand::BigGen::new(55, 100);
    let v: Vec<usize> = big_gen.take(10000).collect();
    println!("Bg Rands = {:?}", v);
}

pub fn insert_sort<T: PartialOrd>(v: &mut [T]) {
    //fewest swaps
    for start in 0..v.len() {
        let mut best = start;
        for i in start..v.len() {
            if v[i] < v[best] {
                best = i;
            }
        }
        v.swap(start, best);
    }
}

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    //1 pass if already sorted
    for start in 0..v.len() {
        let mut sorted = true; //add later
        for i in start..(v.len() - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);
    let mut res = Vec::new();
    let mut b_it = b.into_iter();
    let mut a_it = a.into_iter();
    let mut a_peak = a_it.next();
    let mut b_peak = b_it.next();
    loop {
        match a_peak {
            Some(ref a_val) => match b_peak {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peak.take().unwrap());
                        b_peak = b_it.next();
                    } else {
                        res.push(a_peak.take().unwrap());
                        a_peak = a_it.next();
                    }
                }
                None => {
                    res.push(a_peak.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peak {
                    res.push(b_val)
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    //let mut p = rand::random::<usize>() % v.len();
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);

    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    println!("pre = {:?}", v);
    let p = pivot(v);

    println!("post = {:?}", v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<'a, T>(&'a [T]);

unsafe impl<'a, T> Send for RawSend<'a, T> {}
/*
pub fn threaded_quick_sort<T: 'static + Sync + Send + PartialOrd + std::fmt::Debug>(
    v: &'a mut [T],
) {
    if v.len() <= 1 {
        return;
    }
    println!("pre = {:?}", v);
    let p = pivot(v);

    println!("post = {:?}", v);
    let (a, b): (&'a mut [T], &'a mut [T]) = v.split_at_mut(p);

    let rs: RawSend<'a, T> = RawSend(b);

    let handle = std::thread::spawn(move || {
        threaded_quick_sort::<T>(&mut rs.0);
    });
    threaded_quick_sort(a);
    handle.join().ok();
}
*/
