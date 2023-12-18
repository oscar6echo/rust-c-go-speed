use std::cmp;
use std::time::Instant;

pub fn main() {
    println!("start key-gen-face-five");

    // init
    let mut t: u32; // t=trial key, k=index searched key
    let mut i: usize; // loop counters
    let mut j: usize; // loop counters
    let mut c: usize; // sums counter
    let mut valid: bool; // true if key is valid

    let mut sums = [0; 50000]; // array of all possible sums of key[c[1-5]]

    let mut key = [0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // init keys - empirical

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
                                    let bounded_c = cmp::min(c, sums.len());
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

            i = 0;
            valid = true;

            loop {
                j = i + 1;
                loop {
                    // 1 - test used in debug mode
                    // if sums[i] == sums[j] {

                    // 2 - candidate test for release mode - not efficient
                    // assert!(i < sums.len());
                    // assert!(j < sums.len());
                    // if sums[i] == sums[j] {

                    // 2bis - candidate test for release mode - not efficient
                    // let bounded_i = cmp::min(i, sums.len());
                    // let bounded_j = cmp::min(j, sums.len());
                    // if sums[bounded_i] == sums[bounded_j] {

                    // 3 - test used in release mode - very efficient
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

            // while valid && i < c - 1 {
            //     j = i + 1;
            //     while valid && j < c {
            //         if sums[i] == sums[j] {
            //             valid = false;
            //         }
            //         j += 1;
            //     }
            //     i += 1;
            // }

            // for (i, s) in sums[..c - 1].iter().enumerate() {
            //     if sums[i + 1..c].contains(s) {
            //         valid = false;
            //         break;
            //     }
            // }

            if valid {
                println!("key[{}]={:?}", k, t);
                println!("c={:?}", c);
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
}
