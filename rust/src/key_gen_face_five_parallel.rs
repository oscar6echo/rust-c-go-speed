use std::cmp;
use std::thread;
use std::time::Instant;

const SIZE: usize = 50000;
const LARGE_INT: u32 = 1_000_000_000;
const N_THREAD: usize = 5;

fn search_keys(key_in: [u32; 13], k: usize, t_start: u32, t_end: u32, _i_thread: u32) -> u32 {
    // serch for k-th key with in range t_start..t_end
    // returns candidate key if found else LARGE_INT

    let _start = Instant::now();
    // println!(
    //     "start search_keys in thread={}: k={} t_start={} t_end={}",
    //     _i_thread, k, t_start, t_end,
    // );

    // init
    let mut sums = [0; SIZE]; // array of all possible sums of keys
    let mut c: usize; // sums counter
    let mut i: usize;
    let mut j: usize;

    let mut valid = false;
    let mut t = t_start;

    let mut key = [0; 13];
    for i in 0..k {
        key[i] = key_in[i];
    }

    while !valid && t < t_end {
        key[k] = t;
        let c_max = k + 1;
        c = 0;

        for c1 in 0..c_max {
            for c2 in c1..c_max {
                for c3 in c2..c_max {
                    for c4 in c3..c_max {
                        for c5 in c4..c_max {
                            if c1 != c5 {
                                let bounded_c = cmp::min(c, sums.len() - 1);
                                sums[bounded_c] = key[c1] + key[c2] + key[c3] + key[c4] + key[c5];
                                c += 1;
                            }
                        }
                    }
                }
            }
        }

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

        if valid {
            let _end = Instant::now();
            println!(
                "end search_keys in thread {}: found key[{}]={} in {:?}",
                _i_thread,
                k,
                key[k],
                _end - _start
            );
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
    return 100;
}

pub fn main() {
    // generate keys for faces 1, 2, 3,.., 9, T, J, Q, K, Astart
    // keys are such that the sums of any 2 combinations of 5 faces (with max 4 repetition) are distincts
    // (discarding all other card info)

    println!("start key_gen_face_five_parallel N_THREAD={}", N_THREAD);
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
        for i in 0..N_THREAD {
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
            t_init += N_THREAD as u32 * search_range;
            search_range = build_search_range(t_init);
            // println!("t_init={}", t_init);
        } else {
            key[k] = min_cand_key;
            let end = Instant::now();
            let dt_total = end - start;
            let dt_interm = end - interm;
            println!(
                "key found key[{}]={}, runtime total={:?} last ley={:?}",
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

// output

// start key_gen_face_five_parallel N_THREAD=5
// bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// end search_keys in thread 2: found key[3]=206 in 9.851µs
// end search_keys in thread 1: found key[3]=106 in 11.116µs
// end search_keys in thread 4: found key[3]=406 in 9.802µs
// end search_keys in thread 3: found key[3]=306 in 10.191µs
// end search_keys in thread 0: found key[3]=22 in 19.099µs
// key found key[3]=22, runtime total=1.613376ms last ley=1.613229ms
// end search_keys in thread 2: found key[4]=223 in 17.254µs
// end search_keys in thread 4: found key[4]=423 in 15.954µs
// end search_keys in thread 1: found key[4]=123 in 15.92µs
// end search_keys in thread 0: found key[4]=94 in 151.202µs
// end search_keys in thread 3: found key[4]=323 in 18.256µs
// key found key[4]=94, runtime total=2.578768ms last ley=957.836µs
// end search_keys in thread 3: found key[5]=399 in 61.747µs
// end search_keys in thread 2: found key[5]=312 in 179.504µs
// end search_keys in thread 4: found key[5]=495 in 34.564µs
// key found key[5]=312, runtime total=3.654249ms last ley=1.067406ms
// end search_keys in thread 4: found key[6]=1213 in 88.137µs
// end search_keys in thread 3: found key[6]=1125 in 419.799µs
// end search_keys in thread 2: found key[6]=1062 in 1.015606ms
// end search_keys in thread 1: found key[6]=992 in 1.283956ms
// key found key[6]=992, runtime total=7.400981ms last ley=3.740311ms
// end search_keys in thread 4: found key[7]=2422 in 1.255641ms
// key found key[7]=2422, runtime total=19.718359ms last ley=12.310196ms
// end search_keys in thread 2: found key[8]=5624 in 269.784µs
// key found key[8]=5624, runtime total=56.598123ms last ley=36.872964ms
// end search_keys in thread 4: found key[9]=12616 in 11.568906ms
// end search_keys in thread 3: found key[9]=12522 in 15.858996ms
// key found key[9]=12522, runtime total=188.99877ms last ley=132.391732ms
// end search_keys in thread 4: found key[10]=19998 in 22.246839ms
// key found key[10]=19998, runtime total=406.457957ms last ley=217.452472ms
// runtime = 406.462995ms
// result -> keys=[0, 1, 5, 22, 94, 312, 992, 2422, 5624, 12522, 19998, 0, 0]
