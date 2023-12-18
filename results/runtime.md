|compiler|opt_level|runtime|ratio vs. best|
|--------|---------|-------|--------------|
|   gcc  |   -O3   |  1.54 |      1.0     |
|  clang |   -O3   |  2.27 |     1.47     |
|  clang |   -O1   |  2.3  |     1.49     |
|   go   |         |  2.34 |     1.52     |
|   gcc  |   -O2   |  2.9  |     1.88     |
|   gcc  |   -O1   |  2.95 |     1.92     |
|  clang |   -O2   |  4.35 |     2.82     |
|  rust  |--release|  5.61 |     3.64     |
|   gcc  |         |  10.1 |     6.56     |
|  clang |         | 11.53 |     7.49     |
|  rust  | --debug | 16.62 |     10.79    |