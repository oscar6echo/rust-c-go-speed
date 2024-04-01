use std::{iter::zip, sync::Mutex, thread, time::Instant};

pub fn main() {
    println!("start key-gen-face-seven");

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
        let t_init = key[k - 1] + 1u32;

        let interm = Instant::now();

        let parallel: usize = std::thread::available_parallelism().unwrap().into();
        // on my machine, spawning thread seeme worth it starting at k == 8
        let n: u32 = if k < 7 { 1 } else { parallel as u32 };
        // let n = 4;
        let found = Mutex::new(None);
        let found_ref = &found;
        thread::scope(|s| {
            for offset in 0..n {
                let mut key = key;
                // collects the set of values originally in `sums`
                // value `s` is in the set if the entry `set[s]` contains the current `t` value
                let mut set = vec![];
                // let mut set: Vec<u32> = sets.lock().unwrap().pop().unwrap_or_default();

                let mut task = move || {
                    'outer: for (_i, t) in (t_init + offset..).step_by(n as usize).enumerate() {
                        // if i % 128 == 0 {
                        if let Some(t2) = *found_ref.lock().unwrap() {
                            if t2 < t {
                                break;
                            }
                        }
                        // }
                        key[k] = t;
                        let c_max = k + 1;

                        set.resize(t as usize * 7, 0);
                        // println!("--- t={} bef set={:?}", t, set);

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

                        // println!(">>> t={} aft set={:?}", t, set);

                        let found2 = &mut *found_ref.lock().unwrap();
                        // println!(
                        //     "offset={} t_init={} i={} t={} found2={:?}",
                        //     offset, t_init, i, t, *found2
                        // );

                        match found2 {
                            None => *found2 = Some(t),
                            Some(t2) if *t2 > t => *found2 = Some(t),
                            _ => (),
                        }
                        // println!(
                        //     "offset={} t_init={} i={} t={} found2={:?} after",
                        //     offset, t_init, i, t, *found2
                        // );
                        // sets.lock().unwrap().push(set);

                        // println!("*** return sets(len={:?})", sets.lock().unwrap().len(),);
                        // for (i, x) in sets.lock().unwrap().iter().enumerate() {
                        //     println!("***** {:} -> x={:?}", i, x);
                        // }
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
}

// output

// start key-gen-face-five
// bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// searching for k=3
// key[3]=22
//         runtime for key[3] = 67.268µs
//         runtime for key[3] = 67.99µs
// searching for k=4
// key[4]=98
//         runtime for key[4] = 65.524µs
//         runtime for key[4] = 136.835µs
// searching for k=5
// key[5]=453
//         runtime for key[5] = 273.939µs
//         runtime for key[5] = 414.185µs
// searching for k=6
// key[6]=2031
//         runtime for key[6] = 1.670608ms
//         runtime for key[6] = 2.087724ms
// searching for k=7
// key[7]=8698
//         runtime for key[7] = 2.513979ms
//         runtime for key[7] = 4.604655ms
// searching for k=8
// key[8]=22854
//         runtime for key[8] = 8.987182ms
//         runtime for key[8] = 13.595183ms
// searching for k=9
// key[9]=83661
//         runtime for key[9] = 75.053517ms
//         runtime for key[9] = 88.652728ms
// searching for k=10
// key[10]=262349
//         runtime for key[10] = 365.375451ms
//         runtime for key[10] = 454.035256ms
// searching for k=11
// key[11]=636345
//         runtime for key[11] = 1.297345899s
//         runtime for key[11] = 1.75138527s
// searching for k=12
// key[12]=1479181
//         runtime for key[12] = 5.51781873s
//         runtime for key[12] = 7.269208453s
// runtime = 7.269213215s
