use std::cmp;
use std::time::Instant;

pub fn main() {
    println!("start key-gen-face-five");

    // init
    let mut t: u32; // t=trial key, k=index searched key
    let mut c: usize; // sums counter
    let mut valid: bool; // true if key is valid

    let mut sums = [0; 50000]; // array of all possible sums of key[c[1-5]]
    let mut key = [0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // init keys - empirical

    // let sums_max_idx = sums.len();

    println!("bootstrap -> keys={:?}", key);

    let start = Instant::now();

    for k in 3..11 {
        println!("searching for k={}", k);
        t = key[k - 1] + 1u32;
        let interm = Instant::now();
        valid = false;

        while !valid {
            key[k] = t;
            let c_max = k + 1;
            c = 0;

            for c1 in 0..c_max {
                for c2 in c1..c_max {
                    for c3 in c2..c_max {
                        for c4 in c3..c_max {
                            for c5 in c4..c_max {
                                if c1 != c5 {
                                    // let bounded_c = cmp::min(c, sums_max_idx);
                                    // let bounded_c = cmp::min(c, sums.len() - 1);
                                    let bounded_c = cmp::min(c, sums.len() - 1);
                                    sums[bounded_c] =
                                        key[c1] + key[c2] + key[c3] + key[c4] + key[c5];
                                    c += 1;
                                }
                            }
                        }
                    }
                }
            }

            // for c1 in 0..c_max {
            //     for c2 in c1..c_max {
            //         for c3 in c2..c_max {
            //             for c4 in c3..c_max {
            //                 for c5 in c4..c_max {
            //                     if c1 != c5 {
            //                         unsafe {
            //                             sums[cmp::min(c, sums.len())] = key.get_unchecked(c1)
            //                                 + key.get_unchecked(c2)
            //                                 + key.get_unchecked(c3)
            //                                 + key.get_unchecked(c4)
            //                                 + key.get_unchecked(c5);
            //                             c += 1;
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }

            // v1 - debug mode
            // let mut i: usize;
            // let mut j: usize;
            // valid = true;
            // i = 0;
            // loop {
            //     j = i + 1;
            //     loop {
            //         if sums[i] == sums[j] {
            //             valid = false;
            //         }
            //         j += 1;

            //         if !(valid && j < c) {
            //             break;
            //         }
            //     }
            //     i += 1;
            //     if !(valid && i < c - 1) {
            //         break;
            //     }
            // }

            // v2 -  release mode with unsafe - fastest
            let mut i: usize;
            let mut j: usize;
            i = 0;
            valid = true;
            loop {
                j = i + 1;
                loop {
                    if unsafe { sums.get_unchecked(i) == sums.get_unchecked(j) } {
                        valid = false;
                    }
                    j += 1;

                    if !(valid && j < c) {
                        break;
                    }
                }
                i += 1;
                if !(valid && i < c - 1) {
                    break;
                }
            }

            // v3 - release mode without unsafe - fastest safe
            // let mut i;
            // i = 0;
            // valid = true;
            // loop {
            //     if sums[i + 1..c].contains(&sums[i]) {
            //         valid = false;
            //     }
            //     i += 1;
            //     if !(valid && i < c - 1) {
            //         break;
            //     }
            // }

            // v3b - release mode without unsafe - fatest safe equivalent more compact
            // valid = (0..c - 1).all(|i| !sums[i + 1..c].contains(&sums[i]));

            // v4 - release mode without unsafe - less efficient ways
            // let mut i: usize;
            // i = 0;
            // valid = true;
            // loop {
            //     let sums_i = sums[i];
            //     for &sums_j in &sums[i + 1..c] {
            //         if sums_i == sums_j {
            //             valid = false;
            //             break;
            //         }
            //     }
            //     i += 1;
            //     if !(valid && i < c - 1) {
            //         break;
            //     }
            // }

            // v5 - release mode without unsafe - less efficient ways
            // valid = true;
            // for (i, s) in sums[..c - 1].iter().enumerate() {
            //     if sums[i + 1..c].contains(s) {
            //         valid = false;
            //         break;
            //     }
            // }

            if valid {
                println!("key[{}]={:?}", k, t);
                // println!("c={:?}", c); // ?!?? comment to allow bounded_c = len()-1 without perf impact
                let end = Instant::now();
                println!("\truntime for key[{}] = {:?}", k, (end - interm));
                println!("\truntime for key[{}] = {:?}", k, (end - start));
            } else {
                t += 1;
            }
        }
    }

    let end = Instant::now();
    println!("runtime = {:?}", (end - start));
    println!("key={:?}", key);
}
