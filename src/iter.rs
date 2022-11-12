use crate::*;
pub trait ParallelIterator: Sized + Send {
    type Item: Send;
    fn for_each<F>(self, func: &F, thread_pool: &ThreadPoolDyn)
    where
        F: Fn(&mut Self::Item) + Sync + Send;
}
pub trait ParallelSliceMut<T: Sync + Send> {
    fn par_chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>;
}
pub struct ChunksMut<'data, T: Sync + Send> {
    chunk_size: usize,
    data: &'data mut [T],
}
impl<T: Sync + Send> ParallelSliceMut<T> for &mut [T] {
    fn par_chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T> {
        ChunksMut {
            chunk_size,
            data: self,
        }
    }
}
impl<'data, T: Sync + Send + 'data> ParallelIterator for ChunksMut<'data, T> {
    type Item = &'data mut [T];
    fn for_each<F>(self, func: &F, thread_pool: &ThreadPoolDyn)
    where
        F: Fn(&mut Self::Item) + Sync + Send,
    {
        let mut v = self.data.chunks_mut(self.chunk_size).collect::<Vec<_>>();
        let r = &mut v;
        thread_pool.run(r, func as &(dyn Fn(&mut &'data mut [T]) + Send + Sync));
    }
}
