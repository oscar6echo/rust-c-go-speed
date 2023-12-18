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
│ gcc      ┆           ┆ 10.1    ┆ 6.56           │
│ clang    ┆           ┆ 11.53   ┆ 7.49           │
│ rust     ┆           ┆ 16.62   ┆ 10.79          │
└──────────┴───────────┴─────────┴────────────────┘

```

## Question

The runtime is very sensitive to compiler and compilation flags as demonstrated by the variations within the best compiler (gcc) results.

But I am puzzled by the very poor performance of the rust compiler on this very straightforward program.

Some expert advice is needed to try and improve it.  
Hopefully perf can be brought to par with go, if not gcc best perf.
