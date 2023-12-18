#include <stdio.h>
#include <time.h>

int main()
{
    clock_t clock0, clock1, clocki; // clock_t is defined in <time.h> as int
    int c1, c2, c3, c4, c5;         // test faces running from 0 to 12
    int t, k, validKey;             // t=trial key, k=index searched key, validKey=boolean attribute to t (1=valid)
    int i, j;                       // indexes running through sums
    int sums[50000], c;             // array of all possible sums of key[c1-5]

    int key[13] = {0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}; // init keys - empirical
    k = 3;
    printf("bootstrap: keys = ");
    for (i = 0; i < k; i++)
        printf("%d ", key[i]);
    printf("\n");

    clock0 = clock();

    printf("searching keys from k =%d\n", k);
    // while (k <= 12)
    while (k <= 10)
    { //------------------choose nb keys to search (<=12)
        t = key[k - 1] + 1;
        clocki = clock();
        do
        {
            key[k] = t;
            validKey = 1;
            c = 0;

            for (c1 = 0; c1 <= k; c1++)
            {
                for (c2 = c1; c2 <= k; c2++)
                {
                    for (c3 = c2; c3 <= k; c3++)
                    {
                        for (c4 = c3; c4 <= k; c4++)
                        {
                            for (c5 = c4; c5 <= k; c5++)
                            {
                                for (c5 = c4; c5 <= k; c5++)
                                {
                                    if (c1 != c5)
                                    {
                                        sums[c] = key[c1] + key[c2] + key[c3] + key[c4] + key[c5];
                                        c++;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            i = 0;
            do
            {
                j = i + 1;
                do
                {
                    if (sums[i] == sums[j])
                    {
                        validKey = 0;
                    }
                    j++;
                } while (validKey == 1 && j < c);
                i++;
            } while (validKey == 1 && i < c - 1);

            if (validKey == 1)
            {
                printf("\tkey[%d] = %d\n", k, t);
                clock1 = clock();
                printf("\t\trun time for key[%d] =\t%.2f s\n", k, (float)(clock1 - clocki) / CLOCKS_PER_SEC);
                printf("\t\trun time to key[%d] =\t%.2f s\n", k, (float)(clock1 - clock0) / CLOCKS_PER_SEC);
            }
            else
            {
                t++;
            }
        } while (validKey == 0);
        k++;
    }
    printf("done\n");
    return 0;
}

/*output
gcc -Wall -g -O3 key-gen-face-five.c -o key-gen-face-five-exec
./key-gen-face-five-exec
bootstrap: keys = 0 1 5 
searching keys from k =3
        key[3] = 22
                run time for key[3] =   0.00 s
                run time to key[3] =    0.00 s
        key[4] = 94
                run time for key[4] =   0.00 s
                run time to key[4] =    0.00 s
        key[5] = 312
                run time for key[5] =   0.00 s
                run time to key[5] =    0.00 s
        key[6] = 992
                run time for key[6] =   0.00 s
                run time to key[6] =    0.01 s
        key[7] = 2422
                run time for key[7] =   0.04 s
                run time to key[7] =    0.04 s
        key[8] = 5624
                run time for key[8] =   0.10 s
                run time to key[8] =    0.14 s
        key[9] = 12522
                run time for key[9] =   0.47 s
                run time to key[9] =    0.62 s
        key[10] = 19998
                run time for key[10] =  0.85 s
                run time to key[10] =   1.46 s
        key[11] = 43258
                run time for key[11] =  5.53 s
                run time to key[11] =   7.00 s
        key[12] = 79415
                run time for key[12] =  18.58 s
                run time to key[12] =   25.58 s
done
*/
