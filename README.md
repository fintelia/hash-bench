# hash-bench

A benchmark of the impact of insertion order on Rust's standard library HashMap implementation.

## Results
Running on an Intel Core i5-6600K with 16GB of DDR4 RAM.

```
running 21 tests
running 21 tests
test tests::bench_002K_sorted              ... bench:      39,292 ns/iter (+/- 2,622)
test tests::bench_002K_sorted_during_bench ... bench:      90,059 ns/iter (+/- 3,601)
test tests::bench_002K_unsorted            ... bench:      39,072 ns/iter (+/- 2,469)
test tests::bench_008K_sorted              ... bench:     163,039 ns/iter (+/- 8,752)
test tests::bench_008K_sorted_during_bench ... bench:     363,348 ns/iter (+/- 19,119)
test tests::bench_008K_unsorted            ... bench:     179,937 ns/iter (+/- 14,025)
test tests::bench_032K_sorted              ... bench:   1,028,009 ns/iter (+/- 193,024)
test tests::bench_032K_sorted_during_bench ... bench:   1,906,691 ns/iter (+/- 57,309)
test tests::bench_032K_unsorted            ... bench:   1,178,134 ns/iter (+/- 36,470)
test tests::bench_128K_sorted              ... bench:   4,989,248 ns/iter (+/- 183,860)
test tests::bench_128K_sorted_during_bench ... bench:   8,298,834 ns/iter (+/- 194,107)
test tests::bench_128K_unsorted            ... bench:   6,760,891 ns/iter (+/- 246,155)
test tests::bench_512K_sorted              ... bench:  20,952,505 ns/iter (+/- 468,346)
test tests::bench_512K_sorted_during_bench ... bench:  34,121,880 ns/iter (+/- 656,749)
test tests::bench_512K_unsorted            ... bench:  41,404,297 ns/iter (+/- 1,040,711)
test tests::bench__2M_sorted               ... bench:  81,694,862 ns/iter (+/- 1,347,312)
test tests::bench__2M_sorted_during_bench  ... bench: 133,698,888 ns/iter (+/- 1,058,672)
test tests::bench__2M_unsorted             ... bench: 183,997,158 ns/iter (+/- 6,743,977)
test tests::bench__8M_sorted               ... bench: 320,817,459 ns/iter (+/- 8,146,400)
test tests::bench__8M_sorted_during_bench  ... bench: 546,873,318 ns/iter (+/- 23,351,825)
test tests::bench__8M_unsorted             ... bench: 839,765,298 ns/iter (+/- 25,778,730)

```

![Graph](/graph.png?raw=true)