package main

import (
	"fmt"
	"time"
)

func main() {
	// generate keys for faces 1, 2, 3,.., 9, T, J, Q, K, A
	// keys are such that the sums of any 2 combinations of 5 faces (with max 4 repetition) are distincts
	// (discarding all other card info)

	nbKeys := 10

	var solutions [][]int
	keys := make([]int, nbKeys)

	sumKeys := make([]int, 50000)
	var valid bool

	// bootstrapping
	keys[0] = 0
	keys[1] = 1
	keys[2] = 5
	k := 3
	fmt.Printf("bootstrapping -> k=%d: keys=%v\n", k, keys)

	t0 := time.Now()

	nbKeys = 10
	for k < nbKeys {
		t := keys[k-1] + 1
		valid = false

		for !valid {
			keys[k] = t
			c := 0
			for c1 := 0; c1 < k+1; c1++ {
				for c2 := c1; c2 < k+1; c2++ {
					for c3 := c2; c3 < k+1; c3++ {
						for c4 := c3; c4 < k+1; c4++ {
							for c5 := c4; c5 < k+1; c5++ {
								if c1 != c5 {
									sumKeys[c] = keys[c1] + keys[c2] + keys[c3] + keys[c4] + keys[c5]
									c++

								}
							}

						}
					}
				}
			}

			i := 0
			valid = true
			for (valid) && (i < c-1) {
				j := i + 1
				for (valid) && (j < c) {
					if sumKeys[i] == sumKeys[j] {
						valid = false
						t++
					}
					j++
				}
				i++
			}
		}
		t1 := time.Now()
		dt := t1.Sub(t0)
		fmt.Printf("k=%d: keys=%v - t=%.2f s\n", k, keys, dt.Seconds())
		solutions = append(solutions, keys[:])
		k++
	}

}
