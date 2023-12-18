# C implementation

Commands:

```sh
################# version
gcc --version
# gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0
# Copyright (C) 2021 Free Software Foundation, Inc.
# This is free software; see the source for copying conditions.  There is NO
# warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

clang --version
# Ubuntu clang version 14.0.0-1ubuntu1.1
# Target: x86_64-pc-linux-gnu
# Thread model: posix
# InstalledDir: /usr/bin

################# runtimes
gcc -Wall -g -O3 key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 1.54s

gcc -Wall -g -O2 key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 2.90s

gcc -Wall -g -O1 key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 2.95s

gcc -Wall -g key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 10.10s


clang -Wall -g -O3 key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 2.27s

clang -Wall -g -O2 key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 4.35s

clang -Wall -g -O1 key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 2.30s

clang -Wall -g key-gen-face-five.c -o key-gen-face-five-exe
./key-gen-face-five-exe
# runtime 11.53s

```
