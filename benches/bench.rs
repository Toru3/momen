use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use momen::prelude::*;
use rand::Rng;
use rayon::prelude::*;

fn copy_ref(x: &[f64], y: &mut [f64]) {
    y.iter_mut().zip(x.iter()).for_each(|(y, x)| *y = *x);
}

fn copy_aux(arg: &mut (&[f64], &mut [f64])) {
    copy_ref(arg.0, arg.1);
}

fn copy_rayon(x: &[f64], y: &mut [f64]) {
    y.par_iter_mut()
        .zip(x.par_iter())
        .for_each(|(y, x)| *y = *x);
}

fn gen_rand(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}

fn check(x: &[f64], y: &[f64]) -> bool {
    if x.len() != y.len() {
        return false;
    }
    for (xi, yi) in x.iter().zip(y.iter()) {
        if xi != yi {
            return false;
        }
    }
    true
}

fn bench_copy(c: &mut Criterion) {
    let cool_down = std::time::Duration::from_secs(3);
    std::thread::sleep(cool_down);
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("copy");
    group.plot_config(plot_config);
    let pool1 = ThreadPool::new(black_box(copy_aux));
    let pool2 = ThreadPoolDyn::new();
    for n in 11..22 {
        let size = 1 << n;
        let nproc = std::thread::available_parallelism().unwrap().get();
        let batch_size = (size + nproc - 1) / nproc;
        group.throughput(Throughput::Bytes(
            (2 * std::mem::size_of::<f64>() * size) as u64,
        ));
        {
            let x = gen_rand(size);
            let mut y = vec![0.; size];
            group.bench_with_input(BenchmarkId::new("Reference", size), &size, |b, &_size| {
                b.iter(|| copy_ref(black_box(&x), black_box(&mut y)));
            });
            assert!(check(&x, &y));
            std::thread::sleep(cool_down);
        }
        {
            let x = gen_rand(size);
            let mut y = vec![0f64; size];
            let mut input = x
                .chunks(batch_size)
                .zip(y.chunks_mut(batch_size))
                .collect::<Vec<_>>();
            group.bench_with_input(BenchmarkId::new("ThreadPool", size), &size, |b, &_size| {
                b.iter(|| pool1.run(black_box(&mut input)));
            });
            assert!(check(&x, &y));
            std::thread::sleep(cool_down);
        }
        {
            let x = gen_rand(size);
            let mut y = vec![0f64; size];
            group.bench_with_input(
                BenchmarkId::new("ThreadPoolIter", size),
                &size,
                |b, &_size| {
                    b.iter(|| {
                        x.chunks(batch_size)
                            .zip(y.chunks_mut(batch_size))
                            .par_for_each(&pool1);
                    })
                },
            );
            assert!(check(&x, &y));
            std::thread::sleep(cool_down);
        }
        {
            let x = gen_rand(size);
            let mut y = vec![0f64; size];
            let mut input = x
                .chunks(batch_size)
                .zip(y.chunks_mut(batch_size))
                .collect::<Vec<_>>();
            group.bench_with_input(
                BenchmarkId::new("ThreadPoolDyn", size),
                &size,
                |b, &_size| {
                    b.iter(|| pool2.run(black_box(&mut input), black_box(&copy_aux)));
                },
            );
            assert!(check(&x, &y));
            std::thread::sleep(cool_down);
        }
        {
            let x = gen_rand(size);
            let mut y = vec![0f64; size];
            group.bench_with_input(
                BenchmarkId::new("ThreadPoolDynIter", size),
                &size,
                |b, &_size| {
                    b.iter(|| {
                        x.chunks(batch_size)
                            .zip(y.chunks_mut(batch_size))
                            .par_for_each_dyn(black_box(&copy_aux), &pool2);
                    })
                },
            );
            assert!(check(&x, &y));
            std::thread::sleep(cool_down);
        }
        {
            let x = gen_rand(size);
            let mut y = vec![0.; size];
            group.bench_with_input(BenchmarkId::new("Rayon", size), &size, |b, &_size| {
                b.iter(|| copy_rayon(black_box(&x), black_box(&mut y)));
            });
            assert!(check(&x, &y));
        }
        {
            let x = gen_rand(size);
            let mut y = vec![0.; size];
            group.bench_with_input(BenchmarkId::new("Rayon chunk", size), &size, |b, &_size| {
                b.iter(|| {
                    black_box(&x)
                        .par_chunks(batch_size)
                        .zip(black_box(&mut y).par_chunks_mut(batch_size))
                        .for_each(|(x, y)| copy_ref(x, y));
                });
            });
            assert!(check(&x, &y));
            std::thread::sleep(cool_down);
        }
    }
    group.finish();
}

criterion_group! {
    name = copy;
    config = Criterion::default()
        .sample_size(10000)
        .measurement_time(std::time::Duration::from_secs(60));
    targets = bench_copy
}
criterion_main!(copy);
