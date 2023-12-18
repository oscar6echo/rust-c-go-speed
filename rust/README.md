# Rust implementation

Commands:

```sh
################# version
rustc --version  
# rustc 1.74.1 (a28077b28 2023-12-04)

################# runtimes
cargo build --release

cargo run # debug by default
# start key-gen-face-five
# bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
# searching for k=3
# key[3]=22
# c=52
#         runtime for key[3] = 53.082µs
#         runtime for key[3] = 54.121µs
# searching for k=4
# key[4]=94
# c=121
#         runtime for key[4] = 568.416µs
#         runtime for key[4] = 626.368µs
# searching for k=5
# key[5]=312
# c=246
#         runtime for key[5] = 5.197731ms
#         runtime for key[5] = 5.826828ms
# searching for k=6
# key[6]=992
# c=455
#         runtime for key[6] = 46.230467ms
#         runtime for key[6] = 52.060758ms
# searching for k=7
# key[7]=2422
# c=784
#         runtime for key[7] = 212.627709ms
#         runtime for key[7] = 264.692647ms
# searching for k=8
# key[8]=5624
# c=1278
#         runtime for key[8] = 1.123070044s
#         runtime for key[8] = 1.387766809s
# searching for k=9
# key[9]=12522
# c=1992
#         runtime for key[9] = 5.465622389s
#         runtime for key[9] = 6.853393299s
# searching for k=10
# key[10]=19998
# c=2992
#         runtime for key[10] = 9.773775971s
#         runtime for key[10] = 16.627173322s
# runtime = 16.627177091s

# 16.62 s

cargo run --release  
#     Finished release [optimized] target(s) in 0.00s
#      Running `target/release/rust-runtime-speed-test`
# start key-gen-face-five
# bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
# searching for k=3
# key[3]=22
# c=52
#         runtime for key[3] = 10.584µs
#         runtime for key[3] = 11.605µs
# searching for k=4
# key[4]=94
# c=121
#         runtime for key[4] = 179.625µs
#         runtime for key[4] = 195.057µs
# searching for k=5
# key[5]=312
# c=246
#         runtime for key[5] = 1.535274ms
#         runtime for key[5] = 1.73356ms
# searching for k=6
# key[6]=992
# c=455
#         runtime for key[6] = 16.346825ms
#         runtime for key[6] = 18.086439ms
# searching for k=7
# key[7]=2422
# c=784
#         runtime for key[7] = 74.143575ms
#         runtime for key[7] = 92.235175ms
# searching for k=8
# key[8]=5624
# c=1278
#         runtime for key[8] = 376.954159ms
#         runtime for key[8] = 469.193489ms
# searching for k=9
# key[9]=12522
# c=1992
#         runtime for key[9] = 1.8659372s
#         runtime for key[9] = 2.33513626s
# searching for k=10
# key[10]=19998
# c=2992
#         runtime for key[10] = 3.283270389s
#         runtime for key[10] = 5.618410754s
# runtime = 5.618414151s

# 5.61s

```
