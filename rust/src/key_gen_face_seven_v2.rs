use std::iter::zip;
use std::time::Instant;

pub fn main() {
    // generate keys for faces 1, 2, 3,.., 9, T, J, Q, K, A
    // keys are such that the sums of any 2 combinations of 7 faces (with max 4 repetition) are distincts
    // (discarding all other card info)

    println!("start key-gen-face-seven-v2");

    // init
    let mut t: u32; // t=trial key, k=index searched key
    let mut c: usize; // sums counter
    let mut valid: bool; // true if key is valid

    // collects the set of values originally in `sums`
    // value `s` is in the set if the entry `set[s]` contains the current `t` value
    let mut set = Vec::new();

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
        assert_eq!(key[k - 1] + 1, t + 1);
        t = key[k - 1] + 1;
        let interm = Instant::now();
        valid = false;

        while !valid {
            key[k] = t;
            let c_max = k + 1;
            c = 0;

            valid = true;
            set.resize(t as usize * 7, 0);

            'outer: for (c1, &k1) in zip(0.., key[0..c_max].iter()) {
                for (c2, &k2) in zip(c1.., key[c1..c_max].iter()) {
                    for (c3, &k3) in zip(c2.., key[c2..c_max].iter()) {
                        for (c4, &k4) in zip(c3.., key[c3..c_max].iter()) {
                            for (c5, &k5) in zip(c4.., key[c4..c_max].iter()) {
                                for (c6, &k6) in zip(c5.., key[c5..c_max].iter()) {
                                    for (c7, &k7) in zip(c6.., key[c6..c_max].iter()) {
                                        if (c1 != c5) && (c2 != c6) & (c3 != c7) {
                                            let sum = (k1 + k2 + k3 + k4 + k5 + k6 + k7) as usize;
                                            let e = &mut set[sum];
                                            if *e == t {
                                                valid = false;
                                                break 'outer;
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
    println!("key={:?}", key);
}

// output for k in 3..13

// start key-gen-face-seven-v2
// bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// searching for k=3
// key[3]=22
// c=0
//         runtime for key[3] = 6.903µs
//         runtime for key[3] = 7.697µs
// searching for k=4
// key[4]=98
// c=0
//         runtime for key[4] = 22.239µs
//         runtime for key[4] = 32.881µs
// searching for k=5
// key[5]=453
// c=0
//         runtime for key[5] = 224.114µs
//         runtime for key[5] = 259.576µs
// searching for k=6
// key[6]=2031
// c=0
//         runtime for key[6] = 2.048464ms
//         runtime for key[6] = 2.310766ms
// searching for k=7
// key[7]=8698
// c=0
//         runtime for key[7] = 17.411784ms
//         runtime for key[7] = 19.727984ms
// searching for k=8
// key[8]=22854
// c=0
//         runtime for key[8] = 50.337929ms
//         runtime for key[8] = 70.069866ms
// searching for k=9
// key[9]=83661
// c=0
//         runtime for key[9] = 391.138864ms
//         runtime for key[9] = 461.212276ms
// searching for k=10
// key[10]=262349
// c=0
//         runtime for key[10] = 1.973835869s
//         runtime for key[10] = 2.435051763s
// searching for k=11
// key[11]=636345
// c=0
//         runtime for key[11] = 5.835292573s
//         runtime for key[11] = 8.270348336s
// searching for k=12
// key[12]=1479181
// c=0
//         runtime for key[12] = 20.440023794s
//         runtime for key[12] = 28.710376324s
// runtime = 28.710379477s
// key=[0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181]
