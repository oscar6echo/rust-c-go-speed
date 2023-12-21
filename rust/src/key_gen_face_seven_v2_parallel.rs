// use std::cmp;
use std::iter::zip;
use std::thread;
use std::time::Instant;

// const SIZE: usize = 50000;
const LARGE_INT: u32 = 1_000_000_000;
// const N_THREAD: usize = 5;

fn search_keys(key_in: [u32; 13], k: usize, t_start: u32, t_end: u32, _i_thread: u32) -> u32 {
    // serch for k-th key with in range t_start..t_end
    // returns candidate key if found else LARGE_INT

    let _start = Instant::now();
    // println!(
    //     "start search_keys in thread={}: k={} t_start={} t_end={}",
    //     _i_thread, k, t_start, t_end,
    // );

    // init
    let mut set = Vec::new();

    // let mut sums = [0; SIZE]; // array of all possible sums of keys
    // let mut c: usize; // sums counter
    // let mut i: usize;
    // let mut j: usize;

    let mut valid = false;
    let mut t = t_start;

    let mut key = [0; 13];
    for i in 0..k {
        key[i] = key_in[i];
    }

    while !valid && t < t_end {
        key[k] = t;
        let c_max = k + 1;
        // c = 0;

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
            let _end = Instant::now();
            // println!("key[{}]={:?}", k, t);
            // println!(
            //     "end search_keys in thread {}: found key[{}]={} in {:?}",
            //     _i_thread,
            //     k,
            //     key[k],
            //     _end - _start
            // );
            return key[k];
        } else {
            t += 1;
        }
    }

    return LARGE_INT;
}

// fn build_search_range(t_start: u32) -> u32 {
//     let a = t_start / 40;
//     if a < 100 {
//         return 100;
//     }
//     if a > 20_000 {
//         return 20_000;
//     }
//     return a;
// }

fn build_search_range(_t_start: u32) -> u32 {
    return 1000;
}

pub fn main() {
    // generate keys for faces 1, 2, 3,.., 9, T, J, Q, K, A
    // keys are such that the sums of any 2 combinations of 7 faces (with max 4 repetition) are distincts
    // (discarding all other card info)

    let n_thread_machine: usize = std::thread::available_parallelism().unwrap().into();

    println!(
        "start key_gen_face_seven_parallel n_thread_machine={}",
        n_thread_machine
    );
    let start = Instant::now();
    let mut interm = Instant::now();

    // init
    let mut t_init: u32;

    let mut key = [0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // init keys - empirical
    let k_start = 3;
    println!("bootstrap -> keys={:?}", key);

    t_init = key[k_start - 1] + 1;
    let mut search_range = build_search_range(t_init);
    // println!("search_range={}", search_range);

    let mut k = k_start;
    while k < 13 {
        let mut handles = vec![];
        let n_thread: u32 = if k < 7 {
            1
        } else {
            (n_thread_machine * 0 + 5) as u32
        };
        // let n_thread = n_thread_machine;
        for i in 0..n_thread {
            let handle = thread::spawn(move || {
                let i_thread = i as u32;
                let t_start = t_init + i_thread * search_range;
                let t_end = t_init + (i_thread + 1) * search_range - 1;
                let cand_key = search_keys(key, k, t_start, t_end, i_thread);
                cand_key
            });
            handles.push(handle);
        }

        let mut results: Vec<u32> = vec![];
        for handle in handles {
            results.push(handle.join().unwrap());
        }

        let min_cand_key = *results.iter().min().unwrap();

        if min_cand_key == LARGE_INT {
            // println!("FINAL: no key found for k={} -> carry on", k);
            t_init += n_thread as u32 * search_range;
            search_range = build_search_range(t_init);
            // println!("t_init={}", t_init);
        } else {
            key[k] = min_cand_key;
            let end = Instant::now();
            let dt_total = end - start;
            let dt_interm = end - interm;
            println!(
                "key found key[{}]={}, runtime total={:?} last key={:?}",
                k, key[k], dt_total, dt_interm
            );
            interm = Instant::now();
            k += 1;
            t_init = key[k - 1] + 1;
        }
    }

    let end = Instant::now();
    println!("runtime = {:?}", (end - start));
    println!("result -> keys={:?}", key);
}

// outpout

// start key_gen_face_seven_parallel n_thread_machine=12
// bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// key found key[3]=22, runtime total=192.781µs last key=192.702µs
// key found key[4]=98, runtime total=465.395µs last key=266.202µs
// key found key[5]=453, runtime total=768.496µs last key=297.314µs
// key found key[6]=2031, runtime total=2.624302ms last key=1.848575ms
// key found key[7]=8698, runtime total=10.036859ms last key=7.406595ms
// key found key[8]=22854, runtime total=24.86071ms last key=14.812896ms
// key found key[9]=83661, runtime total=137.439831ms last key=112.568736ms
// key found key[10]=262349, runtime total=703.178043ms last key=565.729447ms
// key found key[11]=636345, runtime total=2.655567192s last key=1.952378324s
// key found key[12]=1479181, runtime total=10.600042894s last key=7.944462437s
// runtime = 10.600052412s
// result -> keys=[0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181]
