use criterion::{black_box, AxisScale, BenchmarkId, Criterion, PlotConfiguration};
use rand::prelude::*;
use std::collections::BTreeMap;

const STEPS: [usize; 6] = [1, 10, 100, 1000, 10_000, 100_000];

pub fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BTreeMap Insert");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for i in STEPS {
        group.bench_function(BenchmarkId::from_parameter(i), |b| {
            let mut rng = StdRng::seed_from_u64(0x1234abcd);
            let mut sl: BTreeMap<usize, usize> =
                std::iter::repeat_with(|| rng.gen()).take(i).collect();

            b.iter(|| {
                sl.insert(rng.gen(), rng.gen());
            })
        });
    }
}

pub fn rand_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("BTreeMap Random Access");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for i in STEPS {
        group.bench_function(BenchmarkId::from_parameter(i), |b| {
            let mut rng = StdRng::seed_from_u64(0x1234abcd);
            let sl: BTreeMap<usize, usize> = std::iter::repeat_with(|| rng.gen())
                .enumerate()
                .take(i)
                .collect();
            let indices: Vec<usize> = std::iter::repeat_with(|| rng.gen_range(0..sl.len()))
                .take(10)
                .collect();

            b.iter(|| {
                for i in &indices {
                    black_box(sl[i]);
                }
            })
        });
    }
}

pub fn iter(c: &mut Criterion) {
    c.bench_function("BTreeMap Iter", |b| {
        let mut rng = StdRng::seed_from_u64(0x1234abcd);
        let sl: BTreeMap<usize, usize> =
            std::iter::repeat_with(|| rng.gen()).take(100_000).collect();

        b.iter(|| {
            for el in &sl {
                black_box(el);
            }
        })
    });
}