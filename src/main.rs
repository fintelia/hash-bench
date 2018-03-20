#![feature(test)]
#![allow(non_snake_case)]

extern crate fnv;
extern crate rahashmap;
extern crate rand;
extern crate test;

#[cfg(test)]
mod tests {
    #[derive(PartialEq, Eq)]
    enum Mode {
        Unsorted,
        Sorted,
        SortDuringBench,
        SortInsertHashed,
        UnsortedInsertHashed,
    }

    use super::*;
    use fnv::FnvBuildHasher;
    use test::Bencher;
    use rahashmap::HashMap;
    use rand::Rng;
    use std::hash::Hasher;
    use std::hash::{BuildHasher, Hash};

    fn bench(b: &mut Bencher, size: usize, mode: Mode) {
        fn sort_values<K: Hash, V, S: BuildHasher>(values: &mut Vec<(K, V)>, s: &S, mask: u64) {
            values.sort_unstable_by_key(|&(ref k, _)| {
                let mut state = s.build_hasher();
                k.hash(&mut state);
                state.finish() & mask
            });
        }

        let s = FnvBuildHasher::default();
        let mask = size.next_power_of_two() - 1;
        let values: Vec<(i64, _)> = {
            let mut rng = rand::thread_rng();
            let mut v = (0..(size as i64)).map(|i| (rng.gen(), i)).collect();
            if mode == Mode::Sorted {
                sort_values(&mut v, &s, mask as u64);
            }
            v
        };

        match mode {
            Mode::Unsorted | Mode::Sorted => b.iter(|| {
                let mut hm = HashMap::with_hasher(s.clone());
                hm.extend(values.clone());
                test::black_box(hm)
            }),
            Mode::SortDuringBench => b.iter(|| {
                let mut values = values.clone();
                sort_values(&mut values, &s, mask as u64);
                let mut hm = HashMap::with_hasher(s.clone());
                hm.extend(values);
                test::black_box(hm)
            }),
            Mode::SortInsertHashed => b.iter(|| {
                let mut values: Vec<_> = values
                    .iter()
                    .map(|&(k, v)| (rahashmap::make_hash(&s, &k), (k, v)))
                    .collect();
                values.sort_unstable_by_key(|&(h, _)| h.inspect() & mask);
                let mut hm = HashMap::with_capacity_and_hasher(size, s.clone());
                for &(h, (k, v)) in values.iter() {
                    hm.insert_hashed_nocheck(h, k, v);
                }
                test::black_box(hm)
            }),
            Mode::UnsortedInsertHashed => b.iter(|| {
                let values: Vec<_> = values
                    .iter()
                    .map(|&(k, v)| (rahashmap::make_hash(&s, &k), (k, v)))
                    .collect();
                let mut hm = HashMap::with_capacity_and_hasher(size, s.clone());
                for &(h, (k, v)) in values.iter() {
                    hm.insert_hashed_nocheck(h, k, v);
                }
                test::black_box(hm)
            }),
        }
    }

    #[bench]
    fn bench_002K_sorted(b: &mut Bencher) {
        bench(b, 2 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench_002K_unsorted(b: &mut Bencher) {
        bench(b, 2 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    fn bench_002K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 2 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench_002K_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 2 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench_002K_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 2 * 1024 + 1, Mode::UnsortedInsertHashed)
    }

    #[bench]
    fn bench_008K_sorted(b: &mut Bencher) {
        bench(b, 8 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench_008K_unsorted(b: &mut Bencher) {
        bench(b, 8 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    fn bench_008K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 8 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench_008K_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 8 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench_008K_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 8 * 1024 + 1, Mode::UnsortedInsertHashed)
    }

    #[bench]
    fn bench_032K_sorted(b: &mut Bencher) {
        bench(b, 32 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench_032K_unsorted(b: &mut Bencher) {
        bench(b, 32 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    fn bench_032K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 32 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench_032K_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 32 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench_032K_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 32 * 1024 + 1, Mode::UnsortedInsertHashed)
    }

    #[bench]
    fn bench_128K_sorted(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench_128K_unsorted(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    #[ignore]
    fn bench_128K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench_128K_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench_128K_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::UnsortedInsertHashed)
    }

    #[bench]
    fn bench_512K_sorted(b: &mut Bencher) {
        bench(b, 512 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench_512K_unsorted(b: &mut Bencher) {
        bench(b, 512 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    #[ignore]
    fn bench_512K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 512 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench_512K_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 512 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench_512K_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 512 * 1024 + 1, Mode::UnsortedInsertHashed)
    }

    #[bench]
    fn bench__2M_sorted(b: &mut Bencher) {
        bench(b, 2 * 1024 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench__2M_unsorted(b: &mut Bencher) {
        bench(b, 2 * 1024 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    #[ignore]
    fn bench__2M_sorted_during_bench(b: &mut Bencher) {
        bench(b, 2 * 1024 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench__2M_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 2 * 1024 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench__2M_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 2 * 1024 * 1024 + 1, Mode::UnsortedInsertHashed)
    }

    #[bench]
    fn bench__8M_sorted(b: &mut Bencher) {
        bench(b, 8 * 1024 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench__8M_unsorted(b: &mut Bencher) {
        bench(b, 8 * 1024 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    #[ignore]
    fn bench__8M_sorted_during_bench(b: &mut Bencher) {
        bench(b, 8 * 1024 * 1024 + 1, Mode::SortDuringBench)
    }
    #[bench]
    fn bench__8M_sorted_insert_hashed(b: &mut Bencher) {
        bench(b, 8 * 1024 * 1024 + 1, Mode::SortInsertHashed)
    }
    #[bench]
    fn bench__8M_unsorted_insert_hashed(b: &mut Bencher) {
        bench(b, 8 * 1024 * 1024 + 1, Mode::UnsortedInsertHashed)
    }
}
