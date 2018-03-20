# hash-bench

A benchmark of the impact of insertion order and hash computation on Rust's standard library HashMap implementation. To gain access to some low level functionality on HashMap, these benchmarks actually use a [fork](https://github.com/fintelia/rahashmap/tree/no-reorder) of the standard library version, though performance should not be impacted.

## Results
Running on an Intel Core i5-6600K with 16GB of DDR4 RAM.

![Graph](/graph.png?raw=true)

```
running 35 tests
test tests::bench_002K_sorted                 ... bench:      25,989 ns/iter (+/- 817)
test tests::bench_002K_sorted_during_bench    ... bench:     188,047 ns/iter (+/- 2,415)
test tests::bench_002K_sorted_insert_hashed   ... bench:      65,611 ns/iter (+/- 7,480)
test tests::bench_002K_unsorted               ... bench:      25,291 ns/iter (+/- 1,379)
test tests::bench_002K_unsorted_insert_hashed ... bench:      18,660 ns/iter (+/- 1,105)
test tests::bench_008K_sorted                 ... bench:     113,132 ns/iter (+/- 4,981)
test tests::bench_008K_sorted_during_bench    ... bench:     927,027 ns/iter (+/- 11,969)
test tests::bench_008K_sorted_insert_hashed   ... bench:     329,144 ns/iter (+/- 10,533)
test tests::bench_008K_unsorted               ... bench:     130,915 ns/iter (+/- 1,699)
test tests::bench_008K_unsorted_insert_hashed ... bench:     105,409 ns/iter (+/- 2,952)
test tests::bench_032K_sorted                 ... bench:     822,597 ns/iter (+/- 15,158)
test tests::bench_032K_sorted_during_bench    ... bench:   4,531,621 ns/iter (+/- 215,618)
test tests::bench_032K_sorted_insert_hashed   ... bench:   1,768,648 ns/iter (+/- 16,843)
test tests::bench_032K_unsorted               ... bench:     986,027 ns/iter (+/- 69,920)
test tests::bench_032K_unsorted_insert_hashed ... bench:     819,163 ns/iter (+/- 52,504)
test tests::bench_128K_sorted                 ... bench:   4,086,533 ns/iter (+/- 192,409)
test tests::bench_128K_sorted_during_bench    ... ignored
test tests::bench_128K_sorted_insert_hashed   ... bench:   8,466,170 ns/iter (+/- 502,833)
test tests::bench_128K_unsorted               ... bench:   5,469,179 ns/iter (+/- 380,908)
test tests::bench_128K_unsorted_insert_hashed ... bench:   4,902,794 ns/iter (+/- 361,578)
test tests::bench_512K_sorted                 ... bench:  17,147,720 ns/iter (+/- 564,594)
test tests::bench_512K_sorted_during_bench    ... ignored
test tests::bench_512K_sorted_insert_hashed   ... bench:  37,497,271 ns/iter (+/- 1,364,904)
test tests::bench_512K_unsorted               ... bench:  33,158,553 ns/iter (+/- 987,269)
test tests::bench_512K_unsorted_insert_hashed ... bench:  25,561,521 ns/iter (+/- 868,525)
test tests::bench__2M_sorted                  ... bench:  66,671,740 ns/iter (+/- 1,306,324)
test tests::bench__2M_sorted_during_bench     ... ignored
test tests::bench__2M_sorted_insert_hashed    ... bench: 158,308,434 ns/iter (+/- 5,433,147)
test tests::bench__2M_unsorted                ... bench: 158,455,060 ns/iter (+/- 5,224,070)
test tests::bench__2M_unsorted_insert_hashed  ... bench: 115,893,118 ns/iter (+/- 2,103,544)
test tests::bench__8M_sorted                  ... bench: 266,762,726 ns/iter (+/- 6,250,259)
test tests::bench__8M_sorted_during_bench     ... ignored
test tests::bench__8M_sorted_insert_hashed    ... bench: 665,298,541 ns/iter (+/- 9,518,462)
test tests::bench__8M_unsorted                ... bench: 639,673,469 ns/iter (+/- 2,691,899)
test tests::bench__8M_unsorted_insert_hashed  ... bench: 483,602,363 ns/iter (+/- 8,671,774)
```
