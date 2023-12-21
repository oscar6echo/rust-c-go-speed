use std::{iter::zip, sync::Mutex, thread, time::Instant};

pub fn main() {
    println!("start key-gen-face-five");

    // init
    let mut t: u32; // t=trial key, k=index searched key

    // let mut sums = [0; 50000]; // array of all possible sums of key[c[1-5]]
    // let sets = &Mutex::<Vec<Vec<u32>>>::default();

    let mut key = [0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // init keys - empirical

    println!("bootstrap -> keys={:?}", key);

    let start = Instant::now();
    t = 5;
    for k in 3..13 {
        println!("searching for k={}", k);
        // sanity check, `t` is always just increasing;
        // An always unique / fresh `t` value is important for correct representation of the
        // `set` array which thus allows us to skip any step of *clear*ing the set
        // since with incremented `t` all entries are considered empty again.
        assert_eq!(key[k - 1] + 1u32, t + 1);
        t = key[k - 1] + 1u32;
        let interm = Instant::now();

        let parallel: usize = std::thread::available_parallelism().unwrap().into();
        // on my machine, spawning thread seeme worth it starting at k == 8
        let n: u32 = if k < 8 { 1 } else { parallel as u32 };
        let found = Mutex::new(None);
        let found_ref = &found;
        thread::scope(|s| {
            for offset in 0..n {
                // let mut set = Vec::<u32>::new();
                // collects the set of values originally in `sums`
                // value `s` is in the set if the entry `set[s]` contains the current `t` value
                let mut set = Vec::<u32>::new();
                // let mut set: Vec<u32> = sets.lock().unwrap().pop().unwrap_or_default();
                let mut task = move || {
                    'outer: for (_i, t) in (t + offset..).step_by(n as usize).enumerate() {
                        if let Some(t2) = *found_ref.lock().unwrap() {
                            if t2 < t {
                                break;
                            }
                        }

                        key[k] = t;
                        let c_max = k + 1;

                        set.resize(t as usize * 5, 0);
                        for (c1, &k1) in zip(0.., key[0..c_max].iter()) {
                            for (c2, &k2) in zip(c1.., key[c1..c_max].iter()) {
                                for (c3, &k3) in zip(c2.., key[c2..c_max].iter()) {
                                    for (c4, &k4) in zip(c3.., key[c3..c_max].iter()) {
                                        for (c5, &k5) in zip(c4.., key[c4..c_max].iter()) {
                                            if c1 != c5 {
                                                let e = &mut set[(k1 + k2 + k3 + k4 + k5) as usize];
                                                if *e == t {
                                                    continue 'outer;
                                                } else {
                                                    *e = t;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let found = &mut *found_ref.lock().unwrap();
                        match found {
                            None => *found = Some(t),
                            Some(t2) if *t2 > t => *found = Some(t),
                            _ => (),
                        }
                        // sets.lock().unwrap().push(set);
                        return;
                    }
                };
                if offset == n - 1 {
                    task();
                } else {
                    s.spawn(task);
                }
            }
        });
        t = found.into_inner().unwrap().unwrap();

        key[k] = t;

        println!("key[{}]={:?}", k, t);
        let end = Instant::now();
        println!("\truntime for key[{}] = {:?}", k, (end - interm));
        println!("\truntime for key[{}] = {:?}", k, (end - start));
    }

    let end = Instant::now();
    println!("runtime = {:?}", (end - start));
    println!("key={:?}", key);
}
