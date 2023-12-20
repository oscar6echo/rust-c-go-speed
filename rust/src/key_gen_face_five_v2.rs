use std::iter::zip;
use std::time::Instant;

pub fn main() {
    // generate keys for faces 1, 2, 3,.., 9, T, J, Q, K, A
    // keys are such that the sums of any 2 combinations of 5 faces (with max 4 repetition) are distincts
    // (discarding all other card info)

    println!("start key-gen-face-five-v2");

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
    for k in 3..11 {
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
            set.resize(t as usize * 5, 0);

            'outer: for (c1, &k1) in zip(0.., key[0..c_max].iter()) {
                for (c2, &k2) in zip(c1.., key[c1..c_max].iter()) {
                    for (c3, &k3) in zip(c2.., key[c2..c_max].iter()) {
                        for (c4, &k4) in zip(c3.., key[c3..c_max].iter()) {
                            for (c5, &k5) in zip(c4.., key[c4..c_max].iter()) {
                                if c1 != c5 {
                                    let sum = (k1 + k2 + k3 + k4 + k5) as usize;
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

// start key-gen-face-five-v2
// bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// searching for k=3
// key[3]=22
// c=0
//         runtime for key[3] = 4.329µs
//         runtime for key[3] = 5.786µs
// searching for k=4
// key[4]=94
// c=0
//         runtime for key[4] = 12.662µs
//         runtime for key[4] = 25.065µs
// searching for k=5
// key[5]=312
// c=0
//         runtime for key[5] = 42.223µs
//         runtime for key[5] = 70.597µs
// searching for k=6
// key[6]=992
// c=0
//         runtime for key[6] = 213.267µs
//         runtime for key[6] = 287.725µs
// searching for k=7
// key[7]=2422
// c=0
//         runtime for key[7] = 631.774µs
//         runtime for key[7] = 923.058µs
// searching for k=8
// key[8]=5624
// c=0
//         runtime for key[8] = 2.123634ms
//         runtime for key[8] = 3.050069ms
// searching for k=9
// key[9]=12522
// c=0
//         runtime for key[9] = 6.746759ms
//         runtime for key[9] = 9.801672ms
// searching for k=10
// key[10]=19998
// c=0
//         runtime for key[10] = 8.912756ms
//         runtime for key[10] = 18.719574ms
// searching for k=11
// key[11]=43258
// c=0
//         runtime for key[11] = 41.005372ms
//         runtime for key[11] = 59.728983ms
// searching for k=12
// key[12]=79415
// c=0
//         runtime for key[12] = 96.160138ms
//         runtime for key[12] = 155.892735ms
// runtime = 155.895923ms
// key=[0, 1, 5, 22, 94, 312, 992, 2422, 5624, 12522, 19998, 43258, 79415]
