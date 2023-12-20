from pathlib import Path

import polars as pl
from py_markdown_table.markdown_table import markdown_table

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
    # gofrom py_markdown_table.markdown_table import markdown_table
    ["go", "", 2.34],
    # rust
    ["rust", "debug v1", 16.61],
    ["rust", "release v2 unsafe", 1.56],
    ["rust", "release v3 safe", 2.20],
    ["rust", "release v3b safe", 2.22],
    ["rust", "release v4 safe", 2.62],
    ["rust", "release v5 safe", 2.89],
    # rust parallel
    ["rust", "release parallel unsafe", 0.41],
]


df = pl.DataFrame(
    data=data,
    schema=["compiler", "opt_level", "runtime"],
)
best = (
    df.filter(~pl.col("opt_level").str.contains("parallel"))
    .select("runtime")
    .min()
    .item()
)
print(best)

col_ratio = "ratio vs. best not parallel"

df = df.with_columns(
    (df["runtime"] / best).round(2).alias(col_ratio),
).sort(col_ratio)

pl.Config.set_tbl_rows(df.shape[0] + 1)

print(df)

path = "runtime.csv"
df.write_csv(path)
print(f"saved {path}")

markdown = markdown_table(df.to_dicts()).set_params(row_sep="markdown").get_markdown()
path = Path("runtime.md")
path.write_text(markdown[3:-3])
print(f"saved {path}")
