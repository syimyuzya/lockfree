# CPU info
```
```
Running on

# Benchmarks
Benchmark code under [benchmark](benchmark) directory.
More rounds per seconds is better.

As you can see, there is a lot to improve!


## THREAD-LOCAL STORAGE
```
Result for 1 threads:
Target 0 (std/global):
mean of 198240753.614 r/s (248171520 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 60616302.961 r/s (76017664 rounds in 1.254 seconds)
Target 2 (blocking with cached access):
mean of 79384829.425 r/s (99491840 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 67775362.856 r/s (84805632 rounds in 1.251 seconds)
Target 4 (lockfree with cached id):
mean of 120921613.733 r/s (151281664 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (std/global):
mean of 351794228.316 r/s (439853056 rounds in 1.250 seconds)
Target 1 (blocking):
mean of 91387834.551 r/s (114702336 rounds in 1.255 seconds)
Target 2 (blocking with cached access):
mean of 128888172.782 r/s (161129472 rounds in 1.250 seconds)
Target 3 (lockfree):
mean of 132565414.200 r/s (166383616 rounds in 1.255 seconds)
Target 4 (lockfree with cached id):
mean of 223314566.964 r/s (280283136 rounds in 1.255 seconds)

Result for 16 threads:
Target 0 (std/global):
mean of 506978660.665 r/s (634090496 rounds in 1.251 seconds)
Target 1 (blocking):
mean of 74016044.065 r/s (92644352 rounds in 1.252 seconds)
Target 2 (blocking with cached access):
mean of 81433862.311 r/s (102024192 rounds in 1.253 seconds)
Target 3 (lockfree):
mean of 145268123.658 r/s (182328320 rounds in 1.255 seconds)
Target 4 (lockfree with cached id):
mean of 236489314.451 r/s (296877056 rounds in 1.255 seconds)

Result for 32 threads:
Target 0 (std/global):
mean of 518824531.111 r/s (649585664 rounds in 1.252 seconds)
Target 1 (blocking):
mean of 60452072.958 r/s (75809792 rounds in 1.254 seconds)
Target 2 (blocking with cached access):
mean of 66846543.370 r/s (83912704 rounds in 1.255 seconds)
Target 3 (lockfree):
mean of 143989350.567 r/s (180112384 rounds in 1.251 seconds)
Target 4 (lockfree with cached id):
mean of 237063050.833 r/s (297663488 rounds in 1.256 seconds)

Result for 128 threads:
Target 0 (std/global):
mean of 507592047.949 r/s (636172288 rounds in 1.253 seconds)
Target 1 (blocking):
mean of 37996092.329 r/s (47751168 rounds in 1.257 seconds)
Target 2 (blocking with cached access):
mean of 45592363.717 r/s (57501696 rounds in 1.261 seconds)
Target 3 (lockfree):
mean of 147015460.824 r/s (184862720 rounds in 1.257 seconds)
Target 4 (lockfree with cached id):
mean of 240908996.410 r/s (302707712 rounds in 1.257 seconds)

```

## QUEUE
```
Result for 1 threads:
Target 0 (mutex vector):
mean of 28331897.702 r/s (35518464 rounds in 1.254 seconds)
Target 1 (mutex linked list):
mean of 7477949.600 r/s (9375744 rounds in 1.254 seconds)
Target 2 (lockfree):
mean of 5642072.747 r/s (7081984 rounds in 1.255 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 9229249.033 r/s (11568128 rounds in 1.253 seconds)
Target 1 (mutex linked list):
mean of 2643828.475 r/s (3306496 rounds in 1.251 seconds)
Target 2 (lockfree):
mean of 2068651.851 r/s (2591744 rounds in 1.253 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7907578.551 r/s (9887744 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 1386508.299 r/s (1739776 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 4426825.858 r/s (6116352 rounds in 1.382 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 4287339.195 r/s (5377024 rounds in 1.254 seconds)
Target 1 (mutex linked list):
mean of 783937.134 r/s (982016 rounds in 1.253 seconds)
Target 2 (lockfree):
mean of 4180263.855 r/s (6435840 rounds in 1.540 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 4612264.652 r/s (5787648 rounds in 1.255 seconds)
Target 1 (mutex linked list):
mean of 820232.985 r/s (1035264 rounds in 1.262 seconds)
Target 2 (lockfree):
mean of 4908368.346 r/s (6220800 rounds in 1.267 seconds)

```

## STACK
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 29549692.275 r/s (37062656 rounds in 1.254 seconds)
Target 1 (mutex linked list):
mean of 7283195.382 r/s (9142272 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 6593735.243 r/s (8275968 rounds in 1.255 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 9250264.620 r/s (11565056 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 2353059.479 r/s (2947072 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 2557201.109 r/s (3210240 rounds in 1.255 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7433090.044 r/s (9293824 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 1368812.569 r/s (1720320 rounds in 1.257 seconds)
Target 2 (lockfree):
mean of 3903283.882 r/s (5470208 rounds in 1.401 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 4842716.115 r/s (6057984 rounds in 1.251 seconds)
Target 1 (mutex linked list):
mean of 780587.895 r/s (978944 rounds in 1.254 seconds)
Target 2 (lockfree):
mean of 3656559.748 r/s (5155840 rounds in 1.410 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 4050709.222 r/s (5082112 rounds in 1.255 seconds)
Target 1 (mutex linked list):
mean of 802002.848 r/s (1015808 rounds in 1.267 seconds)
Target 2 (lockfree):
mean of 3813668.657 r/s (4891648 rounds in 1.283 seconds)

```

## DEQUE
```

Result for 1 threads:
Target 0 (mutex vector):
mean of 26715185.622 r/s (33462272 rounds in 1.253 seconds)
Target 1 (mutex linked list):
mean of 7253376.789 r/s (9104384 rounds in 1.255 seconds)
Target 2 (lockfree):
mean of 2184585.998 r/s (2741248 rounds in 1.255 seconds)

Result for 2 threads:
Target 0 (mutex vector):
mean of 10684810.991 r/s (13358080 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 2555963.319 r/s (3205120 rounds in 1.254 seconds)
Target 2 (lockfree):
mean of 585223.487 r/s (732160 rounds in 1.251 seconds)

Result for 4 threads:
Target 0 (mutex vector):
mean of 7032284.485 r/s (8793088 rounds in 1.250 seconds)
Target 1 (mutex linked list):
mean of 1346665.646 r/s (1686528 rounds in 1.252 seconds)
Target 2 (lockfree):
mean of 1229044.909 r/s (1561600 rounds in 1.271 seconds)

Result for 8 threads:
Target 0 (mutex vector):
mean of 4990660.951 r/s (6246400 rounds in 1.252 seconds)
Target 1 (mutex linked list):
mean of 916036.353 r/s (1150976 rounds in 1.256 seconds)
Target 2 (lockfree):
mean of 1284931.360 r/s (1653760 rounds in 1.287 seconds)

Result for 16 threads:
Target 0 (mutex vector):
mean of 5176563.004 r/s (6478848 rounds in 1.252 seconds)
Target 1 (mutex linked list):
mean of 802201.327 r/s (1012736 rounds in 1.262 seconds)
Target 2 (lockfree):
mean of 1407702.876 r/s (1860608 rounds in 1.322 seconds)

Result for 32 threads:
Target 0 (mutex vector):
mean of 5332452.363 r/s (6700032 rounds in 1.256 seconds)
Target 1 (mutex linked list):
mean of 843685.980 r/s (1065984 rounds in 1.263 seconds)
Target 2 (lockfree):
mean of 1426253.526 r/s (1849344 rounds in 1.297 seconds)

```

## MAP
```
Result for 1 threads:
Target 0 (mutex insert):
mean of 3393818.934 r/s (4248576 rounds in 1.252 seconds)
Target 1 (lockfree insert):
mean of 1288990.340 r/s (1615872 rounds in 1.254 seconds)

Result for 2 threads:
Target 0 (mutex insert):
mean of 1587802.682 r/s (1990656 rounds in 1.254 seconds)
Target 1 (lockfree insert):
mean of 687267.648 r/s (862208 rounds in 1.255 seconds)

Result for 4 threads:
Target 0 (mutex insert):
mean of 926692.215 r/s (1159168 rounds in 1.251 seconds)
Target 1 (lockfree insert):
mean of 1378080.279 r/s (1730560 rounds in 1.256 seconds)

Result for 8 threads:
Target 0 (mutex insert):
mean of 629411.124 r/s (790528 rounds in 1.256 seconds)
Target 1 (lockfree insert):
mean of 2156533.545 r/s (3586048 rounds in 1.663 seconds)

Result for 1 threads:
Target 0 (mutex get):
mean of 3227473.797 r/s (4041728 rounds in 1.252 seconds)
Target 1 (lockfree get):
mean of 2781536.799 r/s (3482624 rounds in 1.252 seconds)

Result for 2 threads:
Target 0 (mutex get):
mean of 1487411.611 r/s (1864704 rounds in 1.254 seconds)
Target 1 (lockfree get):
mean of 3981734.529 r/s (4985856 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex get):
mean of 802960.339 r/s (1004544 rounds in 1.251 seconds)
Target 1 (lockfree get):
mean of 6982476.123 r/s (8766464 rounds in 1.255 seconds)

Result for 8 threads:
Target 0 (mutex get):
mean of 680276.044 r/s (856064 rounds in 1.258 seconds)
Target 1 (lockfree get):
mean of 5206311.877 r/s (6538240 rounds in 1.256 seconds)

Result for 1 threads:
Target 0 (mutex remove):
mean of 3274560.218 r/s (4110336 rounds in 1.255 seconds)
Target 1 (lockfree remove):
mean of 807055.093 r/s (1012736 rounds in 1.255 seconds)

Result for 2 threads:
Target 0 (mutex remove):
mean of 2288736.119 r/s (2862080 rounds in 1.251 seconds)
Target 1 (lockfree remove):
mean of 2136999.183 r/s (2674688 rounds in 1.252 seconds)

Result for 4 threads:
Target 0 (mutex remove):
mean of 1168709.896 r/s (1463296 rounds in 1.252 seconds)
Target 1 (lockfree remove):
mean of 5723562.977 r/s (7188480 rounds in 1.256 seconds)

Result for 8 threads:
Target 0 (mutex remove):
mean of 771100.523 r/s (967680 rounds in 1.255 seconds)
Target 1 (lockfree remove):
mean of 8296406.865 r/s (10416128 rounds in 1.255 seconds)

Result for 1 threads:
Target 0 (mutex mixed):
mean of 3386321.040 r/s (4240384 rounds in 1.252 seconds)
Target 1 (lockfree mixed):
mean of 1126438.577 r/s (1414144 rounds in 1.255 seconds)

Result for 2 threads:
Target 0 (mutex mixed):
mean of 1077257.425 r/s (1347584 rounds in 1.251 seconds)
Target 1 (lockfree mixed):
mean of 414215.132 r/s (520192 rounds in 1.256 seconds)

Result for 4 threads:
Target 0 (mutex mixed):
mean of 526974.686 r/s (661504 rounds in 1.255 seconds)
Target 1 (lockfree mixed):
mean of 720810.258 r/s (909312 rounds in 1.262 seconds)

Result for 8 threads:
Target 0 (mutex mixed):
mean of 362247.503 r/s (456704 rounds in 1.261 seconds)
Target 1 (lockfree mixed):
mean of 911899.781 r/s (1822720 rounds in 1.999 seconds)

```

## MPSC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 295.166632ms
Std's MPSC with 3 threads total time: 284.170339ms
Lockfree MPSC with 3 threads total time: 245.456921ms

Mutexed VecDeque with 5 threads total time: 773.325881ms
Std's MPSC with 5 threads total time: 566.446689ms
Lockfree MPSC with 5 threads total time: 511.845498ms

Mutexed VecDeque with 9 threads total time: 1.640916967s
Std's MPSC with 9 threads total time: 1.103070164s
Lockfree MPSC with 9 threads total time: 991.656694ms

Mutexed VecDeque with 17 threads total time: 3.045002882s
Std's MPSC with 17 threads total time: 2.195532491s
Lockfree MPSC with 17 threads total time: 1.985882905s

Mutexed VecDeque with 33 threads total time: 6.756992998s
Std's MPSC with 33 threads total time: 4.457553496s
Lockfree MPSC with 33 threads total time: 3.910337218s
```

## SPSC CHANNEL
```
Mutexed VecDeque total time: 514.83762ms
Std's MPSC (as SPSC) total time: 145.48426ms
Lockfree SPSC total time: 1.232110602s
```

## SPMC CHANNEL
```
Mutexed VecDeque with 3 threads total time: 253.059379ms
Mutexed Std's MPSC (as SPMC) with 3 threads total time: 158.2498ms
Lockfree SPMC with 3 threads total time: 343.868782ms

Mutexed VecDeque with 5 threads total time: 992.127528ms
Mutexed Std's MPSC (as SPMC) with 5 threads total time: 248.379548ms
Lockfree SPMC with 5 threads total time: 735.321183ms

Mutexed VecDeque with 9 threads total time: 2.547599191s
Mutexed Std's MPSC (as SPMC) with 9 threads total time: 638.790984ms
Lockfree SPMC with 9 threads total time: 849.388493ms

Mutexed VecDeque with 17 threads total time: 3.602655832s
Mutexed Std's MPSC (as SPMC) with 17 threads total time: 1.793397129s
Lockfree SPMC with 17 threads total time: 1.225668208s

Mutexed VecDeque with 33 threads total time: 5.824432184s
Mutexed Std's MPSC (as SPMC) with 33 threads total time: 2.691062621s
Lockfree SPMC with 33 threads total time: 1.770369316s
```

## MPMC CHANNEL
```
Mutexed VecDeque with 4 threads total time: 112.143055ms
Mutexed Std's MPSC (as MPMC)  with 4 threads total time: 153.500455ms
Lockfree MPMC with 4 threads total time: 149.00552ms

Mutexed VecDeque with 8 threads total time: 286.17972ms
Mutexed Std's MPSC (as MPMC)  with 8 threads total time: 836.881322ms
Lockfree MPMC with 8 threads total time: 200.945843ms

Mutexed VecDeque with 16 threads total time: 708.693148ms
Mutexed Std's MPSC (as MPMC)  with 16 threads total time: 2.387672226s
Lockfree MPMC with 16 threads total time: 475.641407ms
```

## REQUEST PROGRAM
```
A program simulating a concurrent server.

Mutexed HashMap and Std's MPSC with 2 threads total time: 2.72278794s
Lockfree structures with 2 threads total time: 2.280682344s

Mutexed HashMap and Std's MPSC with 4 threads total time: 2.535199569s
Lockfree structures with 4 threads total time: 2.504076938s

Mutexed HashMap and Std's MPSC with 8 threads total time: 2.63669388s
Lockfree structures with 8 threads total time: 2.50402761s

Mutexed HashMap and Std's MPSC with 16 threads total time: 2.87111467s
Lockfree structures with 16 threads total time: 2.488030388s
```

## MESSAGE REVERB PROGRAM
```
A program which reverberates messages through a plain queue channel

Mutexed VecDeque with 2 threads total time: 228.73309ms
Mutexed LinkedList with 2 threads total time: 647.987049ms
Lockfree Queue with 2 threads total time: 422.132297ms

Mutexed VecDeque with 4 threads total time: 297.452078ms
Mutexed LinkedList with 4 threads total time: 1.52921817s
Lockfree Queue with 4 threads total time: 238.019769ms

Mutexed VecDeque with 8 threads total time: 405.789305ms
Mutexed LinkedList with 8 threads total time: 2.469540879s
Lockfree Queue with 8 threads total time: 223.587884ms

Mutexed VecDeque with 16 threads total time: 412.271102ms
Mutexed LinkedList with 16 threads total time: 2.350117702s
Lockfree Queue with 16 threads total time: 242.028699ms
```

## HASH MINING
```
A program simulating a hash miner.

Mutexed structures with 2 threads total time: 682.711235ms
Lockfree structures with 2 threads total time: 674.07817ms

Mutexed structures with 4 threads total time: 444.946537ms
Lockfree structures with 4 threads total time: 441.305444ms

Mutexed structures with 8 threads total time: 387.551152ms
Lockfree structures with 8 threads total time: 368.401082ms

Mutexed structures with 16 threads total time: 361.196558ms
Lockfree structures with 16 threads total time: 339.870165ms

Mutexed structures with 32 threads total time: 343.629555ms
Lockfree structures with 32 threads total time: 330.745204ms

Mutexed structures with 64 threads total time: 359.245014ms
Lockfree structures with 64 threads total time: 349.923393ms

Mutexed structures with 128 threads total time: 373.122555ms
Lockfree structures with 128 threads total time: 364.250659ms
```

