//!木綿(momen) is low overhead thread pool library.
//!
//!```rust
//!use momen::prelude::*;
//!fn daxpy(alpha: f64, x: &[f64], y: &mut [f64]) {
//!    y.iter_mut().zip(x.iter()).for_each(|(y, x)| *y += alpha * *x);
//!}
//!let thread_pool = ThreadPoolDyn::new();
//!let n = thread_pool.max_len();
//!let mut x = Vec::with_capacity(1000);
//!let mut y = vec![0f64; 1000];
//!for i in 0..1000 {
//!    x.push(i as f64);
//!}
//!let chunck_size = (1000 + n - 1) / n;
//!let alpha = std::f64::consts::PI;
//!x.chunks(chunck_size)
//! .zip(y.chunks_mut(chunck_size))
//! .par_for_each_dyn(&|(x, y)| daxpy(alpha, x, y), &thread_pool);
//!for i in 0..1000 {
//!    assert_eq!(alpha * x[i], y[i]);
//!}
//!```
pub mod iter {
    use arrayvec::ArrayVec;
    pub trait DynamicParallelIterator: ExactSizeIterator + Sized {
        fn par_for_each_dyn<F>(self, func: &F, thread_pool: &crate::ThreadPoolDyn)
        where
            Self::Item: Send + Sync,
            F: Fn(&mut Self::Item) + Sync + Send,
        {
            let num_threads = thread_pool.max_len();
            assert!(self.len() <= num_threads);
            if num_threads <= 32 {
                let mut v = self.collect::<ArrayVec<_, 32>>();
                thread_pool.run(&mut v, func);
            } else {
                let mut v = self.collect::<Vec<_>>();
                thread_pool.run(&mut v, func);
            }
        }
    }
    impl<T: ExactSizeIterator + Sized> DynamicParallelIterator for T {}
    pub trait StaticParallelIterator: ExactSizeIterator + Sized {
        fn par_for_each<F>(self, thread_pool: &crate::ThreadPool<Self::Item, F>)
        where
            Self::Item: Send + Sync,
            F: Fn(&mut Self::Item) + Clone + Send + Sync + 'static,
        {
            let num_threads = thread_pool.max_len();
            assert!(self.len() <= num_threads);
            if num_threads <= 32 {
                let mut v = self.collect::<ArrayVec<_, 32>>();
                thread_pool.run(&mut v);
            } else {
                let mut v = self.collect::<Vec<_>>();
                thread_pool.run(&mut v);
            }
        }
    }
    impl<T: ExactSizeIterator + Sized> StaticParallelIterator for T {}
}
pub mod prelude {
    pub use crate::iter::{DynamicParallelIterator, StaticParallelIterator};
    pub use crate::{ThreadPool, ThreadPoolDyn};
}
use clone_all::clone_all;
use core::{marker::PhantomData, sync::atomic::AtomicUsize};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

type Futex = core::sync::atomic::AtomicU32;
type FutexInt = u32;
type PhantomUnsync = PhantomData<core::cell::Cell<()>>;

macro_rules! load {
    ($v:expr) => {
        $v.load(core::sync::atomic::Ordering::Acquire)
    };
}
macro_rules! store {
    ($v:expr, $e:expr) => {
        $v.store($e, core::sync::atomic::Ordering::Release)
    };
}
unsafe fn inc(x: &Futex) {
    store!(x, load!(x).wrapping_add(1));
}
fn load_pair(x: &[AtomicUsize; 2]) -> [usize; 2] {
    [load!(x[0]), load!(x[1])]
}
fn store_pair(dst: &[AtomicUsize; 2], x: [usize; 2]) {
    store!(dst[0], x[0]);
    store!(dst[1], x[1]);
}

unsafe fn ref_to_usize<T>(x: &T) -> usize {
    core::mem::transmute(x)
}
unsafe fn usize_to_ref<'a, T>(x: usize) -> &'a mut T {
    core::mem::transmute(x)
}
unsafe fn ref_dyn_fn_to_usize<T: ?Sized>(x: &dyn Fn(&mut T)) -> [usize; 2] {
    core::mem::transmute(x)
}
unsafe fn usize_to_ref_dyn<'a, T: ?Sized>(x: [usize; 2]) -> &'a dyn Fn(&mut T) {
    core::mem::transmute(x)
}

fn init_finish(futex: &Futex) {
    use core::sync::atomic::Ordering::AcqRel;
    if futex.fetch_sub(1, AcqRel) == 1 {
        atomic_wait::wake_one(futex);
    }
}
fn init_wait(futex: &Futex) {
    loop {
        let v = load!(futex);
        if v == 0 {
            break;
        }
        atomic_wait::wait(futex, v);
    }
}
fn wait_cond<P>(futex: &Futex, pred: P) -> FutexInt
where
    P: Fn(FutexInt) -> bool,
{
    let mut val = load!(futex);
    let mut count = 0;
    while !pred(val) {
        if count < 10000 {
            thread::yield_now();
        } else {
            atomic_wait::wait(futex, val);
        }
        count += 1;
        val = load!(futex);
    }
    val
}
struct JobWaiter<'a> {
    count: usize,
    futex: &'a Futex,
    last_job_id: FutexInt,
}
impl<'a> JobWaiter<'a> {
    pub fn new(futex: &'a Futex, init_job_id: FutexInt) -> Self {
        Self {
            count: 0,
            futex,
            last_job_id: init_job_id,
        }
    }
    fn reset(&mut self) {
        self.count = 0;
    }
    fn wait_yield(&mut self, val: FutexInt) {
        if self.count < 10000 {
            thread::yield_now();
        } else {
            atomic_wait::wait(self.futex, val);
        }
        self.count += 1;
    }
    pub fn wait_cond<F, P, T>(&mut self, mut load: F, mut pred: P) -> T
    where
        F: FnMut() -> T,
        P: FnMut(&T) -> bool,
    {
        self.reset();
        loop {
            let new_job_id = load!(self.futex);
            // Almost, `new_job_id > last_job_id`.
            // Rerely, `new_job_id < last_job_id`.
            if new_job_id != self.last_job_id {
                self.last_job_id = new_job_id;
                let v = load();
                if pred(&v) {
                    break v;
                } else {
                    continue;
                }
            }
            self.wait_yield(new_job_id);
        }
    }
}
// 64 is cache line size, avoid false sharing
#[repr(align(64))]
struct Elem {
    futex_end: Futex,
    ptr: AtomicUsize,
}
#[repr(align(64))]
struct FutexWrap(Futex);
#[repr(align(64))]
struct Main {
    futex_start: Futex,
    func: [AtomicUsize; 2],
}
/// static version thread pool
pub struct ThreadPool<T, F>
where
    T: Send + Sync,
    F: Fn(&mut T) + Clone + Send + Sync + 'static,
{
    threads: Vec<(JoinHandle<()>, Arc<Elem>)>,
    futex_start: Arc<FutexWrap>,
    func: F,
    pd: PhantomData<T>,
    unsync: PhantomUnsync,
}
impl<T, F> ThreadPool<T, F>
where
    T: Send + Sync,
    F: Fn(&mut T) + Clone + Send + Sync + 'static,
{
    /// construst thread pool with function
    ///
    /// This function swpawns `nproc - 1` threads.
    pub fn new(f: F) -> Self {
        let cores = core_affinity::get_core_ids().unwrap();
        let (_, t_core) = cores.split_first().unwrap();
        let init_remain = Arc::new(Futex::new(t_core.len() as FutexInt));
        let futex_start = Arc::new(FutexWrap(Futex::new(0)));
        let threads = t_core
            .iter()
            .map(|core| {
                let elem = Arc::new(Elem {
                    futex_end: Futex::new(0),
                    ptr: AtomicUsize::new(0),
                });
                let t = {
                    clone_all!(f, futex_start, elem, core, init_remain);
                    thread::spawn(move || {
                        core_affinity::set_for_current(core);
                        init_finish(&init_remain);
                        let mut waiter = JobWaiter::new(&futex_start.0, 0);
                        loop {
                            let p = waiter.wait_cond(|| load!(elem.ptr), |&p| p != 0);
                            // job execute
                            f(unsafe { usize_to_ref(p) });
                            store!(elem.ptr, 0);
                            store!(elem.futex_end, 1);
                            atomic_wait::wake_one(&elem.futex_end);
                        }
                    })
                };
                (t, elem)
            })
            .collect::<Vec<_>>();
        init_wait(&init_remain);
        Self {
            threads,
            futex_start,
            func: f,
            pd: PhantomData,
            unsync: PhantomData,
        }
    }
    /// returns max length of input of `run()`
    pub fn max_len(&self) -> usize {
        self.threads.len() + 1
    }
    /// run function which given `new()` with `data` on thread pool
    ///
    /// REQUIRE: `data.len() <= self.max_len()`
    pub fn run(&self, data: &mut [T]) {
        let (head, tail) = data.split_first_mut().unwrap();
        let data_len = tail.len();
        assert!(data_len <= self.threads.len(), "data is too long");
        if data_len > 0 {
            self.threads.iter().zip(tail).for_each(|(t, v)| {
                store!(t.1.futex_end, 0); // set not end
                store!(t.1.ptr, unsafe { ref_to_usize(v) }); // set data
            });
            // Worker threads do NOT modify `futex_start`. So, fetch_add is not needed.
            unsafe { inc(&self.futex_start.0) };
            atomic_wait::wake_all(&self.futex_start.0);
        }
        (self.func)(head);
        if data_len > 0 {
            self.threads.iter().take(data_len).for_each(|t| {
                wait_cond(&t.1.futex_end, |x| x == 1);
            });
        }
    }
}
/// dynamic version thread pool
pub struct ThreadPoolDyn {
    threads: Vec<(JoinHandle<()>, Arc<Elem>)>,
    main: Arc<Main>,
    unsync: PhantomUnsync,
}
impl ThreadPoolDyn {
    /// construst thread pool
    ///
    /// This function swpawns `nproc - 1` threads.
    pub fn new() -> Self {
        let cores = core_affinity::get_core_ids().unwrap();
        let (_, t_core) = cores.split_first().unwrap();
        let init_remain = Arc::new(Futex::new(t_core.len() as FutexInt));
        let main = Arc::new(Main {
            futex_start: Futex::new(0),
            func: [AtomicUsize::new(0), AtomicUsize::new(0)],
        });
        let threads = t_core
            .iter()
            .map(|core| {
                let elem = Arc::new(Elem {
                    futex_end: Futex::new(0),
                    ptr: AtomicUsize::new(0),
                });
                let t = {
                    clone_all!(main, elem, core, init_remain);
                    thread::spawn(move || {
                        core_affinity::set_for_current(core);
                        init_finish(&init_remain);
                        let mut waiter = JobWaiter::new(&main.futex_start, 0);
                        loop {
                            let d = waiter.wait_cond(|| load!(elem.ptr), |&p| p != 0);
                            let f = load_pair(&main.func);
                            let end = f[0] == 0 && f[1] == 0;
                            if !end {
                                let d = unsafe { usize_to_ref(d) };
                                let f = unsafe { usize_to_ref_dyn::<core::ffi::c_void>(f) };
                                f(d);
                            }
                            store!(elem.ptr, 0);
                            store!(elem.futex_end, 1);
                            atomic_wait::wake_one(&elem.futex_end);
                            if end {
                                break;
                            }
                        }
                    })
                };
                (t, elem)
            })
            .collect::<Vec<_>>();
        init_wait(&init_remain);
        Self {
            threads,
            main,
            unsync: PhantomData,
        }
    }
    /// returns max length of input of `run()`
    pub fn max_len(&self) -> usize {
        self.threads.len() + 1
    }
    /// run `func` with `data` on thread pool
    ///
    /// REQUIRE: `data.len() <= self.max_len()`
    pub fn run<T: Send + Sync>(&self, data: &mut [T], func: &(dyn Fn(&mut T) + Send + Sync)) {
        debug_assert_eq!(load!(self.main.func[0]), 0);
        debug_assert_eq!(load!(self.main.func[1]), 0);
        let (head, tail) = data.split_first_mut().unwrap();
        let data_len = tail.len();
        assert!(data_len <= self.threads.len(), "data is too long");
        if data_len > 0 {
            let f = unsafe { ref_dyn_fn_to_usize(func) };
            store_pair(&self.main.func, f);
            self.threads.iter().zip(tail).for_each(|(t, v)| {
                store!(t.1.futex_end, 0); // set not end
                store!(t.1.ptr, unsafe { ref_to_usize(v) }); // set data
            });
            // Worker threads do NOT modify `futex_start`. So, fetch_add is not needed.
            unsafe { inc(&self.main.futex_start) };
            atomic_wait::wake_all(&self.main.futex_start);
        }
        func(head);
        if data_len > 0 {
            self.threads.iter().take(data_len).for_each(|t| {
                wait_cond(&t.1.futex_end, |x| x == 1);
            });
            store_pair(&self.main.func, [0, 0]);
        }
        let _ = func;
    }
}
impl Default for ThreadPoolDyn {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for ThreadPoolDyn {
    fn drop(&mut self) {
        // assert_eq!(load!(self.main.func[0]), 0);
        // assert_eq!(load!(self.main.func[1]), 0);
        self.threads.iter().for_each(|t| {
            store!(t.1.futex_end, 0);
            store!(t.1.ptr, usize::MAX);
        });
        // worker threads do NOT modify `futex_start`. So, fetch_add is not needed.
        unsafe { inc(&self.main.futex_start) };
        atomic_wait::wake_all(&self.main.futex_start);
        thread::yield_now();
        self.threads.iter().for_each(|t| {
            wait_cond(&t.1.futex_end, |x| x == 1);
        });
        // assert_eq!(self.threads.iter().filter(|t| !t.0.is_finished()).count(), 0);
    }
}
