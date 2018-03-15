#![feature(test)]
#![allow(non_snake_case)]

extern crate test;
extern crate rand;


#[cfg(test)]
mod tests {
    #[derive(PartialEq, Eq)]
    enum Mode {
        Unsorted,
        Sorted,
        SortDuringBench,
    }


    use super::*;
    use test::Bencher;
    use rand::Rng;
    use std::hash::Hasher;
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, BuildHasher};

    fn sort_values<K: Hash, V, S: BuildHasher>(values: &mut Vec<(K, V)>, s: &S, mask: u64) {
        values.sort_by_key(|&(ref k, _)| {
            let mut state = s.build_hasher();
            k.hash(&mut state);
            state.finish() & mask
        });
    }

    fn bench(b: &mut Bencher, size: usize, mode: Mode) {
        let s = RandomState::new();
        let mut rng = rand::thread_rng();
        let mut values: Vec<(i64, _)> = (0..(size as i64)).map(|i| (rng.gen(), i)).collect();
        let mask = (size as u64).next_power_of_two() - 1;

        if mode == Mode::Sorted {
            sort_values(&mut values, &s, mask);
        }

        b.iter(|| {
            if mode == Mode::SortDuringBench {
                sort_values(&mut values, &s, mask);
            }

            let mut hm = HashMap::with_hasher(s.clone());
            hm.extend(values.clone());
            test::black_box(hm)
        });
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
    fn bench_128K_sorted(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::Sorted)
    }
    #[bench]
    fn bench_128K_unsorted(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::Unsorted)
    }
    #[bench]
    fn bench_128K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 128 * 1024 + 1, Mode::SortDuringBench)
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
    fn bench_512K_sorted_during_bench(b: &mut Bencher) {
        bench(b, 512 * 1024 + 1, Mode::SortDuringBench)
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
    fn bench__2M_sorted_during_bench(b: &mut Bencher) {
        bench(b, 2 * 1024 * 1024 + 1, Mode::SortDuringBench)
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
    fn bench__8M_sorted_during_bench(b: &mut Bencher) {
        bench(b, 8 * 1024 * 1024 + 1, Mode::SortDuringBench)
    }
}
