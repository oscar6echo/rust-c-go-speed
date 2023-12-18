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
                                    sums[c] = key[c1] + key[c2] + key[c3] + key[c4] + key[c5];
                                    c += 1;
                                }
                            }
                        }
                    }
                }
            }

            // unsafe {
            //     for c1 in 0..k {
            //         for c2 in c1..=k {
            //             for c3 in c2..=k {
            //                 for c4 in c3..=k {
            //                     for c5 in c4..=k {
            //                         if c1 != c5 {
            //                             sums[c] = key.get_unchecked(c1)
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

            while valid && i < c - 1 {
                j = i + 1;
                while valid && j < c {
                    if sums[i] == sums[j] {
                        valid = false;
                    }
                    j += 1;
                }
                i += 1;
            }

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
        // k += 1;
    }

    let end = Instant::now();
    println!("runtime = {:?}", (end - start));
}
