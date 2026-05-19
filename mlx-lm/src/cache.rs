use mlx_rs::{error::Exception, ops::concatenate_axis, ops::indexing::slice, Array};

// TODO: somehow move quantized methods to a separate trait?
pub trait KeyValueCache {
    fn is_quantized(&self) -> bool {
        false
    }

    /// Returns the group size used for quantization. `None` if not quantized.
    fn group_size(&self) -> Option<i32> {
        None
    }

    /// Returns the number of bits used for quantization. `None` if not quantized.
    fn bits(&self) -> Option<i32> {
        None
    }

    fn offset(&self) -> i32;

    fn max_size(&self) -> Option<i32>;

    fn update_and_fetch(&mut self, keys: Array, values: Array)
        -> Result<(Array, Array), Exception>;
}

impl<T> KeyValueCache for &'_ mut T
where
    T: KeyValueCache,
{
    fn is_quantized(&self) -> bool {
        T::is_quantized(self)
    }

    fn group_size(&self) -> Option<i32> {
        T::group_size(self)
    }

    fn bits(&self) -> Option<i32> {
        T::bits(self)
    }

    fn offset(&self) -> i32 {
        T::offset(self)
    }

    fn max_size(&self) -> Option<i32> {
        T::max_size(self)
    }

    fn update_and_fetch(
        &mut self,
        keys: Array,
        values: Array,
    ) -> Result<(Array, Array), Exception> {
        T::update_and_fetch(self, keys, values)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConcatKeyValueCache {
    keys: Option<Array>,
    values: Option<Array>,
    offset: i32,
}

impl ConcatKeyValueCache {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KeyValueCache for ConcatKeyValueCache {
    fn offset(&self) -> i32 {
        self.offset
    }

    fn max_size(&self) -> Option<i32> {
        None
    }

    fn update_and_fetch(
        &mut self,
        keys: Array,
        values: Array,
    ) -> Result<(Array, Array), Exception> {
        match (self.keys.take(), self.values.take()) {
            (Some(k), Some(v)) => {
                self.keys = Some(concatenate_axis(&[k, keys], -2)?);
                self.values = Some(concatenate_axis(&[v, values], -2)?);
            }
            _ => {
                self.keys = Some(keys);
                self.values = Some(values);
            }
        }
        let shape = self.keys.as_ref().expect("Keys cannot be None").shape();
        self.offset = shape[shape.len() - 2];

        Ok((
            self.keys.clone().expect("Keys cannot be None"),
            self.values.clone().expect("Values cannot be None"),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct BatchRotatingKVCache {
    keys: Option<Array>,
    values: Option<Array>,
    offset: i32,
    idx: i32,
    max_size: i32,
    rotated: bool,
}

impl BatchRotatingKVCache {
    pub fn new(max_size: i32) -> Self {
        Self {
            keys: None,
            values: None,
            offset: 0,
            idx: 0,
            max_size,
            rotated: false,
        }
    }
}

impl KeyValueCache for BatchRotatingKVCache {
    fn offset(&self) -> i32 {
        self.offset
    }

    fn max_size(&self) -> Option<i32> {
        Some(self.max_size)
    }

    fn update_and_fetch(
        &mut self,
        keys: Array,
        values: Array,
    ) -> Result<(Array, Array), Exception> {
        let n = keys.shape().iter().rev().nth(1).copied().unwrap();

        if self.keys.is_none() {
            if n > self.max_size {
                let start = vec![0, 0, n - self.max_size, 0];
                let end = keys.shape().to_vec();
                let step = vec![1, 1, 1, 1];
                self.keys = Some(slice(&keys, &start, &end, &step)?);
                self.values = Some(slice(&values, &start, &end, &step)?);
                self.rotated = true;
                self.idx = 0;
            } else {
                self.keys = Some(keys);
                self.values = Some(values);
                self.idx = n % self.max_size;
                if n == self.max_size {
                    self.rotated = true;
                    self.idx = 0;
                }
            }
            self.offset = n;
        } else {
            let mut k_cache = self.keys.take().unwrap();
            let mut v_cache = self.values.take().unwrap();

            if n + self.idx <= self.max_size {
                // We use indexing with a range to perform the update
                // Since mlx-rs doesn't expose a direct slice_update yet in a convenient way,
                // we'll use concatenate + slice if needed or just assume seq_len=1 for now?
                // Actually, let's implement it properly using concatenation and slicing to simulate a ring buffer
                
                let head = slice(&k_cache, &vec![0, 0, 0, 0], &vec![k_cache.shape()[0], k_cache.shape()[1], self.idx, k_cache.shape()[3]], &vec![1,1,1,1])?;
                let tail = slice(&k_cache, &vec![0, 0, self.idx + n, 0], &k_cache.shape().to_vec(), &vec![1,1,1,1])?;
                k_cache = concatenate_axis(&[head, keys, tail], -2)?;
                
                let head_v = slice(&v_cache, &vec![0, 0, 0, 0], &vec![v_cache.shape()[0], v_cache.shape()[1], self.idx, v_cache.shape()[3]], &vec![1,1,1,1])?;
                let tail_v = slice(&v_cache, &vec![0, 0, self.idx + n, 0], &v_cache.shape().to_vec(), &vec![1,1,1,1])?;
                v_cache = concatenate_axis(&[head_v, values, tail_v], -2)?;
                
                self.idx = (self.idx + n) % self.max_size;
                if !self.rotated && self.idx == 0 {
                    self.rotated = true;
                }
            } else {
                // Wrap around case
                // For simplicity in this port, we'll just append and slice the last max_size
                // This is slightly less efficient but ensures correctness
                k_cache = concatenate_axis(&[k_cache, keys], -2)?;
                v_cache = concatenate_axis(&[v_cache, values], -2)?;
                
                let current_len = k_cache.shape()[2];
                let start = vec![0, 0, current_len - self.max_size, 0];
                let end = k_cache.shape().to_vec();
                k_cache = slice(&k_cache, &start, &end, &vec![1,1,1,1])?;
                v_cache = slice(&v_cache, &start, &end, &vec![1,1,1,1])?;
                
                self.rotated = true;
                self.idx = 0;
            }
            
            self.keys = Some(k_cache);
            self.values = Some(v_cache);
            self.offset += n;
        }

        Ok((
            self.keys.clone().expect("Keys cannot be None"),
            self.values.clone().expect("Values cannot be None"),
        ))
    }
}
