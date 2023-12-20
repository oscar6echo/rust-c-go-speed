|   algo  |compiler|       opt_level       |runtime|best vs. naive & not parallel|
|---------|--------|-----------------------|-------|-----------------------------|
|optimized|  rust  |      release safe     |  0.19 |             0.12            |
|  naive  |  rust  |release parallel unsafe|  0.41 |             0.27            |
|  naive  |   gcc  |          -O3          |  1.54 |             1.0             |
|  naive  |  rust  |   release v2 unsafe   |  1.56 |             1.01            |
|  naive  |  rust  |    release v3 safe    |  2.2  |             1.43            |
|  naive  |  rust  |    release v3b safe   |  2.22 |             1.44            |
|  naive  |  clang |          -O3          |  2.27 |             1.47            |
|  naive  |  clang |          -O1          |  2.3  |             1.49            |
|  naive  |   go   |                       |  2.34 |             1.52            |
|  naive  |  rust  |    release v4 safe    |  2.62 |             1.7             |
|  naive  |   gcc  |          -O2          |  2.9  |             1.88            |
|  naive  |  rust  |    release v5 safe    |  2.89 |             1.88            |
|  naive  |   gcc  |          -O1          |  2.95 |             1.92            |
|  naive  |  clang |          -O2          |  4.35 |             2.82            |
|  naive  |   gcc  |                       |  10.1 |             6.56            |
|  naive  |  clang |                       | 11.53 |             7.49            |
|  naive  |  rust  |        debug v1       | 16.61 |            10.79            |