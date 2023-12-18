import polars as pl

data = [
    # gcc
    ["gcc", "-O3", 1.54],
    ["gcc", "-O2", 2.90],
    ["gcc", "-O1", 2.95],
    ["gcc", "", 10.10],
    # clang
    ["clang", "-O3", 2.27],
    ["clang", "-O2", 4.35],
    ["clang", "-O1", 2.30],
    ["clang", "", 11.53],
    # go
    ["go", "", 2.34],
    # rust
    ["rust", "", 16.62],
]


df = pl.DataFrame(
    data=data,
    schema=["compiler", "opt_level", "runtime"],
)
best = df.select("runtime").min().item()
print(best)

df = df.with_columns(
    (df["runtime"] / best).round(2).alias("ratio vs. best"),
).sort("ratio vs. best")

pl.Config.set_tbl_rows(df.shape[0] + 1)

print(df)
df.write_csv("runtime.csv")
