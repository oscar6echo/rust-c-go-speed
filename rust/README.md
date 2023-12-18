# Rust implementation

Commands:

```sh
################# version
rustc --version  
# rustc 1.74.1 (a28077b28 2023-12-04)

################# runtimes

cargo run # debug by default
# start key-gen-face-five
# bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
# searching for k=3
# key[3]=22
# c=52
#         runtime for key[3] = 52.549µs
#         runtime for key[3] = 53.589µs
# searching for k=4
# key[4]=94
# c=121
#         runtime for key[4] = 616.588µs
#         runtime for key[4] = 673.581µs
# searching for k=5
# key[5]=312
# c=246
#         runtime for key[5] = 5.37487ms
#         runtime for key[5] = 6.050961ms
# searching for k=6
# key[6]=992
# c=455
#         runtime for key[6] = 48.128094ms
#         runtime for key[6] = 54.184387ms
# searching for k=7
# key[7]=2422
# c=784
#         runtime for key[7] = 215.553062ms
#         runtime for key[7] = 269.741423ms
# searching for k=8
# key[8]=5624
# c=1278
#         runtime for key[8] = 1.164053299s
#         runtime for key[8] = 1.433799025s
# searching for k=9
# key[9]=12522
# c=1992
#         runtime for key[9] = 5.449962709s
#         runtime for key[9] = 6.883765923s
# searching for k=10
# key[10]=19998
# c=2992
#         runtime for key[10] = 9.728687693s
#         runtime for key[10] = 16.612457769s
# runtime = 16.612461305s

# 16.61 s

# unsafe mode
cargo run --release  
# start key-gen-face-five
# bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
# searching for k=3
# key[3]=22
# c=52
#         runtime for key[3] = 7.416µs
#         runtime for key[3] = 8.337µs
# searching for k=4
# key[4]=94
# c=121
#         runtime for key[4] = 72.749µs
#         runtime for key[4] = 84.651µs
# searching for k=5
# key[5]=312
# c=246
#         runtime for key[5] = 532.954µs
#         runtime for key[5] = 620.21µs
# searching for k=6
# key[6]=992
# c=455
#         runtime for key[6] = 5.159732ms
#         runtime for key[6] = 5.782516ms
# searching for k=7
# key[7]=2422
# c=784
#         runtime for key[7] = 22.865601ms
#         runtime for key[7] = 28.653722ms
# searching for k=8
# key[8]=5624
# c=1278
#         runtime for key[8] = 108.064411ms
#         runtime for key[8] = 136.722763ms
# searching for k=9
# key[9]=12522
# c=1992
#         runtime for key[9] = 498.795883ms
#         runtime for key[9] = 635.522569ms
# searching for k=10
# key[10]=19998
# c=2992
#         runtime for key[10] = 933.640526ms
#         runtime for key[10] = 1.569167043s
# runtime = 1.569170502s

# 1.56s


# safe mode
cargo run --release  
# start key-gen-face-five
# bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
# searching for k=3
# key[3]=22
# c=52
#         runtime for key[3] = 11.154µs
#         runtime for key[3] = 13.005µs
# searching for k=4
# key[4]=94
# c=121
#         runtime for key[4] = 123.478µs
#         runtime for key[4] = 141.723µs
# searching for k=5
# key[5]=312
# c=246
#         runtime for key[5] = 1.13136ms
#         runtime for key[5] = 1.27991ms
# searching for k=6
# key[6]=992
# c=455
#         runtime for key[6] = 9.277923ms
#         runtime for key[6] = 10.563149ms
# searching for k=7
# key[7]=2422
# c=784
#         runtime for key[7] = 31.166671ms
#         runtime for key[7] = 41.734232ms
# searching for k=8
# key[8]=5624
# c=1278
#         runtime for key[8] = 149.229778ms
#         runtime for key[8] = 190.96847ms
# searching for k=9
# key[9]=12522
# c=1992
#         runtime for key[9] = 718.673224ms
#         runtime for key[9] = 909.645708ms
# searching for k=10
# key[10]=19998
# c=2992
#         runtime for key[10] = 1.294074792s
#         runtime for key[10] = 2.203724495s
# runtime = 2.20372748s

# 2.20s
```
