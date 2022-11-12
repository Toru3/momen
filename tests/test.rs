use rand::Rng;

fn gen_rand(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}

fn daxpy(alpha: f64, x: &[f64], y: &mut [f64]) {
    y.iter_mut()
        .zip(x.iter())
        .for_each(|(y, x)| *y += alpha * *x);
}
fn daxpy_aux(arg: &mut (&[f64], &mut [f64])) {
    daxpy(std::f64::consts::PI, arg.0, arg.1);
}

#[test]
fn test_static() {
    let thread_pool = momen::ThreadPool::new(daxpy_aux);
    let n = thread_pool.max_len();
    let len = 1_000_000;
    for _ in 0..1000 {
        let x = gen_rand(len);
        let mut y = vec![0f64; len];
        let chunck_size = (len + n - 1) / n;
        let mut v = x
            .chunks(chunck_size)
            .zip(y.chunks_mut(chunck_size))
            .collect::<Vec<_>>();
        thread_pool.run(&mut v);
        for i in 0..1000 {
            assert_eq!(std::f64::consts::PI * x[i], y[i]);
        }
    }
}
#[test]
fn test_dyn() {
    let thread_pool = momen::ThreadPoolDyn::new();
    let n = thread_pool.max_len();
    let len = 1_000_000;
    for _ in 0..1000 {
        let x = gen_rand(len);
        let mut y = vec![0f64; len];
        let chunck_size = (len + n - 1) / n;
        let mut v = x
            .chunks(chunck_size)
            .zip(y.chunks_mut(chunck_size))
            .collect::<Vec<_>>();
        let alpha = rand::random();
        thread_pool.run(&mut v, &|(x, y)| daxpy(alpha, x, y));
        for i in 0..1000 {
            assert_eq!(alpha * x[i], y[i]);
        }
    }
}
