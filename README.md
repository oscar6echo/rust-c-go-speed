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
┌──────────┬─────────────────────────┬─────────┬─────────────────────────────┐
│ compiler ┆ opt_level               ┆ runtime ┆ ratio vs. best not parallel │
│ ---      ┆ ---                     ┆ ---     ┆ ---                         │
│ str      ┆ str                     ┆ f64     ┆ f64                         │
╞══════════╪═════════════════════════╪═════════╪═════════════════════════════╡
│ rust     ┆ release parallel unsafe ┆ 0.41    ┆ 0.27                        │
│ gcc      ┆ -O3                     ┆ 1.54    ┆ 1.0                         │
│ rust     ┆ release v2 unsafe       ┆ 1.56    ┆ 1.01                        │
│ rust     ┆ release v3 safe         ┆ 2.2     ┆ 1.43                        │
│ rust     ┆ release v3b safe        ┆ 2.22    ┆ 1.44                        │
│ clang    ┆ -O3                     ┆ 2.27    ┆ 1.47                        │
│ clang    ┆ -O1                     ┆ 2.3     ┆ 1.49                        │
│ go       ┆                         ┆ 2.34    ┆ 1.52                        │
│ rust     ┆ release v4 safe         ┆ 2.62    ┆ 1.7                         │
│ gcc      ┆ -O2                     ┆ 2.9     ┆ 1.88                        │
│ rust     ┆ release v5 safe         ┆ 2.89    ┆ 1.88                        │
│ gcc      ┆ -O1                     ┆ 2.95    ┆ 1.92                        │
│ clang    ┆ -O2                     ┆ 4.35    ┆ 2.82                        │
│ gcc      ┆                         ┆ 10.1    ┆ 6.56                        │
│ clang    ┆                         ┆ 11.53   ┆ 7.49                        │
│ rust     ┆ debug v1                ┆ 16.61   ┆ 10.79                       │
└──────────┴─────────────────────────┴─────────┴─────────────────────────────┘
```

Not in competition but interesting:  
This program can be made parallel in steps: Several searches can be run in parallel on distinct ranges, then the main thread collects the candidates keys and keep the smallest, if any. The the next step takes place.

## Question

Cf. forum question <https://users.rust-lang.org/t/rust-vs-c-vs-go-runtime-speed-comparison/104107>

Conversation with the community brought the rust runtime from x10 to 1x the best C runtime ! - in 6 hours !! I'm seriously impressed :clap:
