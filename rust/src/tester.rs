use std::{iter::zip, sync::Mutex, thread, time::Instant};

pub fn main() {
    println!("start test");

    // init
    let mut t: u32; // t=trial key, k=index searched key

    // let mut sums = [0; 50000]; // array of all possible sums of key[c[1-5]]
    let sets = &Mutex::<Vec<Vec<u32>>>::default();

    let mut key = [0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // init keys - empirical

    println!("bootstrap -> keys={:?}", key);

    let start = Instant::now();
    t = 5;
    for k in 3..5 {
        println!("searching for k={}", k);
        // sanity check, `t` is always just increasing;
        // An always unique / fresh `t` value is important for correct representation of the
        // `set` array which thus allows us to skip any step of *clear*ing the set
        // since with incremented `t` all entries are considered empty again.
        assert_eq!(key[k - 1] + 1u32, t + 1);
        let t_init = key[k - 1] + 1u32;
        let interm = Instant::now();

        // let parallel: usize = std::thread::available_parallelism().unwrap().into();
        // on my machine, spawning thread seeme worth it starting at k == 8
        // let n: u32 = if k < 8 { 1 } else { parallel as u32 };
        // let n: u32 = parallel as u32;
        let n: u32 = 4 as u32;
        let found = Mutex::new(None);
        let found_ref = &found;
        // let found_ref = &Mutex::new(None);
        thread::scope(|s| {
            for offset in 0..n {
                // collects the set of values originally in `sums`
                // value `s` is in the set if the entry `set[s]` contains the current `t` value
                println!("offset={}", offset);
                let mut set = vec![];
                // let mut set = sets.lock().unwrap().pop().unwrap_or_default();
                // println!("set={:?}", set);
                let task = move || {
                    'outer: for (i, t) in (t_init + offset..).step_by(n as usize).enumerate() {
                        if let Some(x) = *found_ref.lock().unwrap() {
                            if x < t {
                                println!(
                                    "early break offset={} t_init={} i={} x={} t={}",
                                    offset, t_init, i, x, t
                                );
                                break;
                            }
                        }
                        // println!("offset={} t_init={} i={} t={}", offset, t_init, i, t);
                        key[k] = t;
                        let c_max = k + 1;

                        set.resize(t as usize * 7, 0);

                        for (c1, &k1) in zip(0.., key[0..c_max].iter()) {
                            for (c2, &k2) in zip(c1.., key[c1..c_max].iter()) {
                                for (c3, &k3) in zip(c2.., key[c2..c_max].iter()) {
                                    for (c4, &k4) in zip(c3.., key[c3..c_max].iter()) {
                                        for (c5, &k5) in zip(c4.., key[c4..c_max].iter()) {
                                            for (c6, &k6) in zip(c5.., key[c5..c_max].iter()) {
                                                for (c7, &k7) in zip(c6.., key[c6..c_max].iter()) {
                                                    if (c1 != c5) && (c2 != c6) & (c3 != c7) {
                                                        let sum = (k1 + k2 + k3 + k4 + k5 + k6 + k7)
                                                            as usize;
                                                        let e = &mut set[sum];
                                                        if *e == t {
                                                            // println!("offset={} t_init={} i={} t={} exit", offset, t_init, i, t);
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
                            }
                        }

                        let found2 = &mut *found_ref.lock().unwrap();

                        println!(
                            "offset={} t_init={} i={} t={} found2={:?}",
                            offset, t_init, i, t, *found2
                        );

                        match found2 {
                            None => {
                                *found2 = {
                                    println!(
                                        "offset={} t_init={} i={} match None t={}",
                                        offset, t_init, i, t
                                    );
                                    Some(t)
                                }
                            }
                            Some(x) => {
                                println!(
                                    "offset={} t_init={} i={} match Some(x) x={} t={}",
                                    offset, t_init, i, x, t
                                );
                                if *x > t {
                                    *found2 = Some(t)
                                }
                            }
                            _ => (),
                        }

                        // println!("found2={:?} ", *found2);
                        println!(
                            "offset={} found2={:?} sets.len()={}",
                            offset,
                            *found2,
                            sets.lock().unwrap().len(),
                        );
                        sets.lock().unwrap().push(set);
                        return;
                    }
                };
                if offset == n - 1 {
                    println!("start task offset={} in main thread", offset);
                    task();
                } else {
                    println!("start task offset={} in new thread", offset);
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
