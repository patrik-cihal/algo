use std::ops::{Range, RangeBounds};

pub struct SegmentTree<T, F> {
    n: usize,
    data: Vec<T>,
    neutral: T,
    f: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, neutral: T, f: F) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            n: size,
            data: vec![neutral.clone(); 2 * size],
            neutral,
            f,
        }
    }

    pub fn new_from_iter<I>(iter: I, neutral: T, f: F) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let v: Vec<_> = iter.into_iter().collect();
        let mut size = 1;
        while size < v.len() {
            size <<= 1;
        }
        let mut data = vec![neutral.clone(); size << 1];
        for i in 0..v.len() {
            data[size + i] = v[i].clone();
        }
        for i in (1..size).rev() {
            data[i] = (f)(&data[i<<1], &data[(i<<1) + 1]);
        }
        Self {
            n: size,
            data,
            neutral,
            f,
        }
    }

    pub fn update(&mut self, i: usize, value: T) {
        let mut i = i + self.n;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            self.data[i] = (self.f)(&self.data[i<<1], &self.data[(i<<1) + 1]);
        }
    }

    pub fn get(&self, i: usize) -> &T {
        &self.data[i + self.n]
    }

    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        self.query_inner(range, 1, ..)
    }

    fn to_range(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&p) => p,
            std::ops::Bound::Excluded(&p) => p+1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&p) => p+1,
            std::ops::Bound::Excluded(&p) => p,
            std::ops::Bound::Unbounded => self.n,
        };
        start..end
    }

    fn query_inner(&self, range: impl RangeBounds<usize>, i: usize, inner_range: impl RangeBounds<usize>) -> T {
        let range = self.to_range(range);
        let inner_range = self.to_range(inner_range);

        if range.start >= range.end {
            return self.neutral.clone();
        }
        if range.start == inner_range.start && range.end == inner_range.end {
            return self.data[i].clone();
        }
        let mid = (inner_range.start + inner_range.end) >> 1;
        let left = self.query_inner(range.start..mid.min(range.end), i << 1, inner_range.start..mid);
        let right = self.query_inner(mid.max(range.start)..range.end, (i << 1) + 1, mid..inner_range.end);
        (self.f)(&left, &right) 
    }

    pub fn data(&self) -> &[T] {
        &self.data[self.n..]
    }
}

/// Segment Tree with Lazy Propagation
/// 
/// # Arguments
/// 
/// * `T` - Data type
/// 
/// * `F` - Function to merge data
/// 
/// * `G` - Function to update data
/// 
/// * `H` - Function to merge lazy
/// 
/// # Example
/// ```
/// use algo_lib::collections::SegmentTreeRU;
/// 
/// let mut seg = SegmentTreeRU::new_from_iter(0..10, 0, |&d1, &d2| a + b, |&d, &l, len| d + l * len as i32, |&l1, &l2| l1 + l2);
/// 
/// seg.update_range(2..5, 1);
/// assert_eq!(seg.query(2..5), 3);
/// 
/// seg.update_range(2..5, 1);
/// assert_eq!(seg.query(2..5), 6);
/// 
/// ```
pub struct SegmentTreeRU<T, F, G, H> {
    n: usize, 
    data: Vec<T>,
    lazy: Vec<Option<T>>, 
    neutral: T,
    f: F,
    g: G,
    h: H
}

impl<T, F, G, H> SegmentTreeRU<T, F, G, H> where T: Clone+Copy, F: Fn(&T, &T) -> T, G: Fn(&T, &T, usize) -> T, H: Fn(&T, &T) -> T {
    pub fn new(n: usize, neutral: T, f: F, g: G, h: H) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            n: size,
            data: vec![neutral.clone(); size*2],
            lazy: vec![None; size*2],
            neutral,
            f,
            g,
            h
        }
    }

    pub fn new_from_iter<I>(iter: I, neutral: T, f: F, g: G, h: H) -> Self where I: IntoIterator<Item = T> {
        let v: Vec<_> = iter.into_iter().collect();
        let mut size = 1;
        while size < v.len() {
            size *= 2;
        }
        let mut data = vec![neutral.clone(); 2 * size];
        for (i, val) in v.into_iter().enumerate() {
            data[size + i] = val;
        }
        for i in (1..size).rev() {
            data[i] = (f)(&data[2 * i], &data[2 * i + 1]);
        }
        Self {
            n: size,
            data,
            lazy: vec![None; 2*size],
            neutral,
            f,
            g,
            h,
        }
    }

    pub fn query(&mut self, range: impl RangeBounds<usize>) -> T {
        self.query_inner(self.to_range(range), 1, self.to_range(..))
    }

    fn propagate(&mut self, i: usize, inner_range: Range<usize>) {
        let Some(lazy) = self.lazy[i].clone() else {
            return;
        };
        self.data[i] = (self.g)(&self.data[i], &lazy, inner_range.end-inner_range.start);
        if inner_range.end - inner_range.start != 1 {
            if let Some(child_lazy) = &self.lazy[i*2] {
                self.lazy[i*2] = Some((self.h)(child_lazy, &lazy));
            } else {
                self.lazy[i*2] = Some(lazy.clone());
            }
            if let Some(child_lazy) = &self.lazy[i*2+1] {
                self.lazy[i*2+1] = Some((self.h)(child_lazy, &lazy));
            } else {
                self.lazy[i*2+1] = Some(lazy.clone());
            }
        }
        self.lazy[i] = None;
    }

    fn query_inner(&mut self, range: Range<usize>, i: usize, inner_range: Range<usize>) -> T {
        self.propagate(i, inner_range.clone());

        if range.start >= inner_range.end || inner_range.start >= range.end {
            return self.neutral.clone();
        }
        if range.start <= inner_range.start && inner_range.end <= range.end {
            return self.data[i];
        }

        let mid = (inner_range.start+inner_range.end)/2;
        let left = self.query_inner(range.clone(), i*2, inner_range.start..mid);
        let right = self.query_inner(range, i*2+1, mid..inner_range.end);
        (self.f)(&left, &right)
    }

    pub fn update(&mut self, range: impl RangeBounds<usize>, val: T) {
        self.update_inner(self.to_range(range), val, 1, self.to_range(..));
    }
    
    fn update_inner(&mut self, range: Range<usize>, val: T, i: usize, inner_range: Range<usize>) {
        self.propagate(i, inner_range.clone());

        if range.end <= inner_range.start || inner_range.end <= range.start {
            return;
        }

        if range.start <= inner_range.start && inner_range.end <= range.end {
            self.lazy[i] = Some(val.clone());
            self.propagate(i, inner_range);
            return;
        }

        let mid = (inner_range.start+inner_range.end)/2;
        self.update_inner(range.clone(), val, i*2, inner_range.start..mid);
        self.update_inner(range, val, i*2+1, mid..inner_range.end);

        self.data[i] = (self.f)(&self.data[i*2], &self.data[i*2+1]);
    }

    fn to_range(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&p) => p,
            std::ops::Bound::Excluded(&p) => p+1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&p) => p+1,
            std::ops::Bound::Excluded(&p) => p,
            std::ops::Bound::Unbounded => self.n,
        };
        start..end
    }
}