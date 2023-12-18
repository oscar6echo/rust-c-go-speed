# Go implementation

Commands:

```sh
################# version
go version                   
go version go1.20.1 linux/amd64

################# runtimes
go run ./key-gen-face-five.go
# bootstrapping -> k=3: keys=[0 1 5 0 0 0 0 0 0 0]
# k=3: keys=[0 1 5 22 0 0 0 0 0 0] - t=0.00 s
# k=4: keys=[0 1 5 22 94 0 0 0 0 0] - t=0.00 s
# k=5: keys=[0 1 5 22 94 312 0 0 0 0] - t=0.00 s
# k=6: keys=[0 1 5 22 94 312 992 0 0 0] - t=0.02 s
# k=7: keys=[0 1 5 22 94 312 992 2422 0 0] - t=0.09 s
# k=8: keys=[0 1 5 22 94 312 992 2422 5624 0] - t=0.47 s
# k=9: keys=[0 1 5 22 94 312 992 2422 5624 12522] - t=2.34 s
# 2.34s

```
