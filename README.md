木綿(momen) is low overhead thread pool library.
木綿(momen) means cotton in Japanese.

# Usage
```rust
fn daxpy(alpha: f64, x: &[f64], y: &mut [f64]) {
    y.iter_mut().zip(x.iter()).for_each(|(y, x)| *y += alpha * *x);
}
let thread_pool = momen::ThreadPoolDyn::new();
let n = thread_pool.max_len();
let mut x = Vec::with_capacity(1000);
let mut y = vec![0f64; 1000];
for i in 0..1000 {
    x.push(i as f64);
}
let chunck_size = (1000 + n - 1) / n;
let mut v = x.chunks(chunck_size).zip(y.chunks_mut(chunck_size)).collect::<Vec<_>>();
let alpha = std::f64::consts::PI;
thread_pool.run(&mut v, &|(x, y)| daxpy(alpha, x, y));
for i in 0..1000 {
    assert_eq!(alpha * x[i], y[i]);
}
```

# benchmark
<img src="image/lines.png">
* OS : Ubuntu
* CPU : Ryzen 9 5950X
* MEM : DDR4 3600MHz 128GB
* momen = "0.1.0"
* rayon = "1.5.3"
