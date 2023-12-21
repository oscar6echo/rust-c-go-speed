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
┌───────────┬──────────┬─────────────────────────┬─────────┬───────────────────────────────┐
│ algo      ┆ compiler ┆ opt_level               ┆ runtime ┆ best vs. naive & not parallel │
│ ---       ┆ ---      ┆ ---                     ┆ ---     ┆ ---                           │
│ str       ┆ str      ┆ str                     ┆ f64     ┆ f64                           │
╞═══════════╪══════════╪═════════════════════════╪═════════╪═══════════════════════════════╡
│ optimized ┆ rust     ┆ release parallel safe b ┆ 0.08    ┆ 0.05                          │
│ optimized ┆ rust     ┆ release parallel safe a ┆ 0.1     ┆ 0.06                          │
│ optimized ┆ rust     ┆ release safe            ┆ 0.19    ┆ 0.12                          │
│ naive     ┆ rust     ┆ release parallel unsafe ┆ 0.48    ┆ 0.31                          │
│ naive     ┆ gcc      ┆ -O3                     ┆ 1.54    ┆ 1.0                           │
│ naive     ┆ rust     ┆ release v2 unsafe       ┆ 1.56    ┆ 1.01                          │
│ naive     ┆ rust     ┆ release v3 safe         ┆ 2.2     ┆ 1.43                          │
│ naive     ┆ rust     ┆ release v3b safe        ┆ 2.22    ┆ 1.44                          │
│ naive     ┆ clang    ┆ -O3                     ┆ 2.27    ┆ 1.47                          │
│ naive     ┆ clang    ┆ -O1                     ┆ 2.3     ┆ 1.49                          │
│ naive     ┆ go       ┆                         ┆ 2.34    ┆ 1.52                          │
│ naive     ┆ rust     ┆ release v4 safe         ┆ 2.62    ┆ 1.7                           │
│ naive     ┆ gcc      ┆ -O2                     ┆ 2.9     ┆ 1.88                          │
│ naive     ┆ rust     ┆ release v5 safe         ┆ 2.89    ┆ 1.88                          │
│ naive     ┆ gcc      ┆ -O1                     ┆ 2.95    ┆ 1.92                          │
│ naive     ┆ clang    ┆ -O2                     ┆ 4.35    ┆ 2.82                          │
│ naive     ┆ gcc      ┆                         ┆ 10.1    ┆ 6.56                          │
│ naive     ┆ clang    ┆                         ┆ 11.53   ┆ 7.49                          │
│ naive     ┆ rust     ┆ debug v1                ┆ 16.61   ┆ 10.79                         │
└───────────┴──────────┴─────────────────────────┴─────────┴───────────────────────────────┘
```

2 parallel versions - see scripts.

```sh
┌───────────┬──────────┬─────────────────────────┬─────────┐
│ algo      ┆ compiler ┆ desc                    ┆ runtime │
│ ---       ┆ ---      ┆ ---                     ┆ ---     │
│ str       ┆ str      ┆ str                     ┆ f64     │
╞═══════════╪══════════╪═════════════════════════╪═════════╡
│ naive     ┆ rust     ┆ safe 5-card             ┆ 0.26    │
│ optimized ┆ rust     ┆ safe 5-card parallel  a ┆ 0.08    │
│ optimized ┆ rust     ┆ safe 5-card parallel  b ┆ 0.05    │
│ optimized ┆ rust     ┆ safe 7-card parallel  a ┆ 10.6    │
│ optimized ┆ rust     ┆ safe 7-card parallel  b ┆ 7.3     │
└───────────┴──────────┴─────────────────────────┴─────────┘
```

## Question

Cf. forum question <https://users.rust-lang.org/t/rust-vs-c-vs-go-runtime-speed-comparison/104107>

Conversation with the community brought the rust runtime from x10 to 1x the best C runtime ! - in 6 hours.  
Then the next day to x0.12 ! And the next: a parallel version ! Total: a x200 speedup and several clever recipes in rust `_v2_` algos.  
I'm seriously impressed :clap:  

Special mention to [steffhan](https://users.rust-lang.org/u/steffahn/summary) for the optimized algo and efficient parallel version.
