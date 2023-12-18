# Rust vs. C vs. Go runtime speed comparison

## Program

Simple numerical program with integers, fixed sized arrays, nested loops.  
(Note: It computes some first keys for a Poker hand evaluator)  

Implemented in:

+ [Rust](./rust/src/key_gen_face_five.rs)
+ [C](./c/key-gen-face-five.c)
+ [Go](./go/key-gen-face-five.go)

## System

```sh
# hardware overview
# OS: Ubuntu 22.04.3 LTS x86_64 
# Host: NUC10i7FNH M38010-308 
# Kernel: 6.2.0-39-generic 
# Shell: zsh 5.8.1 
# DE: GNOME 42.9 
# Terminal: gnome-terminal 
# CPU: Intel i7-10710U (12) @ 4.700GHz 
# GPU: Intel Comet Lake UHD Graphics 
# Memory: 31800MiB  
```

## Results

```sh
┌──────────┬───────────┬─────────┬────────────────┐
│ compiler ┆ opt_level ┆ runtime ┆ ratio vs. best │
│ ---      ┆ ---       ┆ ---     ┆ ---            │
│ str      ┆ str       ┆ f64     ┆ f64            │
╞══════════╪═══════════╪═════════╪════════════════╡
│ gcc      ┆ -O3       ┆ 1.54    ┆ 1.0            │
│ clang    ┆ -O3       ┆ 2.27    ┆ 1.47           │
│ clang    ┆ -O1       ┆ 2.3     ┆ 1.49           │
│ go       ┆           ┆ 2.34    ┆ 1.52           │
│ gcc      ┆ -O2       ┆ 2.9     ┆ 1.88           │
│ gcc      ┆ -O1       ┆ 2.95    ┆ 1.92           │
│ clang    ┆ -O2       ┆ 4.35    ┆ 2.82           │
│ rust     ┆ --release ┆ 5.61    ┆ 3.64           │
│ gcc      ┆           ┆ 10.1    ┆ 6.56           │
│ clang    ┆           ┆ 11.53   ┆ 7.49           │
│ rust     ┆ --debug   ┆ 16.62   ┆ 10.79          │
└──────────┴───────────┴─────────┴────────────────┘

```

## Question

Cf. forum question <https://users.rust-lang.org/t/rust-vs-c-vs-go-runtime-speed-comparison/104107>
