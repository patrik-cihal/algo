use std::ops::{Range, RangeBounds};

pub struct SegTree<D: Clone, F: Fn(D, D) -> D> {
    n: usize,
    data: Vec<Option<D>>,
    m: F
}

impl<D: Clone, M: Fn(D, D) -> D> SegTree<D, M> {
    pub fn new(n: usize, f: M) -> Self {
        Self {
            data: vec![None; n*4],
            n,
            m: f
        }
    }

    fn build(&mut self, init_data: &Vec<Option<D>>, ti: usize, tl: usize, tr: usize) {
        if tl+1 == tr {
            self.data[ti] = init_data[tl].clone();
            return;
        }
        let tm = (tl+tr)/2;
        self.build(init_data, ti*2, tl, tm);
        self.build(init_data, ti*2+1, tm, tr);
        self.data[ti] = self.merge(self.data[ti*2].clone(), self.data[ti*2+1].clone());
    }

    pub fn from_iter<I: IntoIterator<Item=D>>(iter: I, f: M) -> Self {
        Self::from_iter_option(iter.into_iter().map(|x| Some(x)), f)
    }

    pub fn from_iter_option<I: IntoIterator<Item=Option<D>>>(iter: I, f: M) -> Self {
        let init_data = iter.into_iter().collect::<Vec<_>>();
        let mut result = Self::new(init_data.len(), f);
        result.build(&init_data, 1, 0, init_data.len());
        result
    }

    fn merge(&self, a: Option<D>, b: Option<D>) -> Option<D> {
        let Some(a) = a else {
            return b;
        };
        let Some(b) = b else {
            return Some(a);
        };
        Some((self.m)(a, b))
    }

    fn to_range(&self, range_bounds: impl RangeBounds<usize>) -> Range<usize> {
        let start = match range_bounds.start_bound() {
            std::ops::Bound::Included(&p) => p,
            std::ops::Bound::Excluded(&p) => p + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range_bounds.end_bound() {
            std::ops::Bound::Included(&p) => p + 1,
            std::ops::Bound::Excluded(&p) => p,
            std::ops::Bound::Unbounded => self.n,
        };
        start..end
    }

    fn query_inner(&self, l: usize, r: usize, ti: usize, tl: usize, tr: usize) -> Option<D> {
        if l >= r {
            return None;
        }
        if tl == l && tr == r {
            return self.data[ti].clone();
        }

        let tm = (tl+tr)/2;
        let left_res = self.query_inner(l, r.min(tm), ti*2, tl, tm);
        let right_res = self.query_inner(l.max(tm), r, ti*2+1, tm, tr);

        self.merge(left_res, right_res)
    }

    pub fn query(&self, range_bounds: impl RangeBounds<usize>) -> D {
        let range = self.to_range(range_bounds);
        self.query_inner(range.start, range.end, 1, 0, self.n).unwrap()
    }

    fn update_inner(&mut self, i: usize, val: D, ti: usize, tl: usize, tr: usize) {
        if tl+1 == tr {
            assert!(i == tl);
            self.data[ti] = Some(val);
            return;
        }
        let tm = (tl+tr)/2;
        if i < tm {
            self.update_inner(i, val, ti*2, tl, tm);
        }
        else {
            self.update_inner(i, val, ti*2+1, tm, tr);
        };
        self.data[ti] = self.merge(self.data[ti*2].clone(), self.data[ti*2+1].clone());
    }

    pub fn update(&mut self, i: usize, val: D) {
        self.update_inner(i, val, 1, 0, self.n);
    }
}

pub struct LazySegTree<D: Clone, L: Clone, M: Fn(D, D) -> D, ML: Fn(L, L) -> L, U: Fn(D, L) -> D> {
    n: usize,
    data: Vec<Option<D>>,
    lazy: Vec<Option<L>>,
    m: M,
    ml: ML,
    u: U
}

impl<D: Clone, L: Clone, M: Fn(D, D) -> D, ML: Fn(L, L) -> L, U: Fn(D, L) -> D> LazySegTree<D, L, M, ML, U> {
    pub fn new(n: usize, m: M, ml: ML, u: U) -> Self {
        Self {
            data: vec![None; n*4],
            lazy: vec![None; n*4],
            n,
            m,
            ml,
            u
        }
    }

    fn build(&mut self, init_data: &Vec<Option<D>>, ti: usize, tl: usize, tr: usize) {
        if tl+1 == tr {
            self.data[ti] = init_data[tl].clone();
            return;
        }
        let tm = (tl+tr)/2;
        self.build(init_data, ti*2, tl, tm);
        self.build(init_data, ti*2+1, tm, tr);
        self.data[ti] = self.merge(self.data[ti*2].clone(), self.data[ti*2+1].clone());
    }

    pub fn from_iter<I: IntoIterator<Item=D>>(iter: I, m: M, ml: ML, u: U) -> Self {
        Self::from_iter_option(iter.into_iter().map(|x| Some(x)), m, ml, u)
    }

    pub fn from_iter_option<I: IntoIterator<Item=Option<D>>>(iter: I, m: M, ml: ML, u: U) -> Self {
        let init_data = iter.into_iter().collect::<Vec<_>>();
        let mut result = Self::new(init_data.len(), m, ml, u);
        result.build(&init_data, 1, 0, init_data.len());
        result
    }

    fn merge(&self, a: Option<D>, b: Option<D>) -> Option<D> {
        let Some(a) = a else {
            return b;
        };
        let Some(b) = b else {
            return Some(a);
        };
        Some((self.m)(a, b))
    }

    fn merge_lazy(&self, l1: Option<L>, l2: Option<L>) -> Option<L> {
        let Some(l1) = l1 else {
            return l2;
        };
        let Some(l2) = l2 else {
            return Some(l1);
        };
        Some((self.ml)(l1, l2))
    }

    fn unite(&self, d: Option<D>, l: Option<L>) -> Option<D> {
        let Some(l) = l else {
            return d;
        };
        let Some(d) = d else {
            return None;
        };
        Some((self.u)(d, l))
    }

    fn to_range(&self, range_bounds: impl RangeBounds<usize>) -> Range<usize> {
        let start = match range_bounds.start_bound() {
            std::ops::Bound::Included(&p) => p,
            std::ops::Bound::Excluded(&p) => p + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range_bounds.end_bound() {
            std::ops::Bound::Included(&p) => p + 1,
            std::ops::Bound::Excluded(&p) => p,
            std::ops::Bound::Unbounded => self.n,
        };
        start..end
    }

    fn push(&mut self, ti: usize) {
        let lazy = self.lazy[ti].take();
        self.data[ti] = self.unite(self.data[ti].clone(), lazy.clone());
        if ti * 2 >= self.data.len() {
            return;
        }
        self.lazy[ti*2] = self.merge_lazy(self.lazy[ti*2].clone(), lazy.clone());
        self.lazy[ti*2+1] = self.merge_lazy(self.lazy[ti*2+1].clone(), lazy.clone());
    }

    fn query_inner(&mut self, l: usize, r: usize, ti: usize, tl: usize, tr: usize) -> Option<D> {
        self.push(ti);

        if l >= r {
            return None;
        }

        if tl == l && tr == r {
            return self.data[ti].clone();
        }

        let tm = (tl+tr)/2;
        let left_res = self.query_inner(l, r.min(tm), ti*2, tl, tm);
        let right_res = self.query_inner(l.max(tm), r, ti*2+1, tm, tr);

        self.data[ti] = self.merge(self.data[ti*2].clone(), self.data[ti*2+1].clone());

        self.merge(left_res, right_res)
    }

    pub fn query(&mut self, range_bounds: impl RangeBounds<usize>) -> D {
        let range = self.to_range(range_bounds);
        self.query_inner(range.start, range.end, 1, 0, self.n).unwrap()
    }

    fn update_inner(&mut self, l: usize, r: usize, val: L, ti: usize, tl: usize, tr: usize) {
        self.push(ti);
        if l >= r {
            return;
        }
        if tl == l && tr == r {
            self.lazy[ti] = Some(val);
            self.push(ti);
            return;
        }

        let tm = (tl+tr)/2;
        self.update_inner(l, r.min(tm), val.clone(), ti*2, tl, tm);
        self.update_inner(l.max(tm), r, val, ti*2+1, tm, tr);

        self.data[ti] = self.merge(self.data[ti*2].clone(), self.data[ti*2+1].clone());
    }

    pub fn update(&mut self, range_bounds: impl RangeBounds<usize>, val: L) {
        let range = self.to_range(range_bounds);
        self.update_inner(range.start, range.end, val, 1, 0, self.n);
    }
}


mod seg_tree_tests {
    use super::SegTree;

    #[test]
    fn query_test() {
        let mx = 1000;
        let st = SegTree::from_iter(0..mx, |a, b| a+b);
        for l in 0..mx {
            for r in l+1..=mx {
                let tar_sum = ((r-1)*r)/2 - (l.max(1)-1)*l/2;
                assert_eq!(st.query(l..r), tar_sum);
            }
        }
    }
    #[test]
    fn upd_test() {
        let mx = 1000;
        let mut st = SegTree::from_iter(0..mx as i64, |a, b| a+b);
        for i in 0..mx {
            st.update(i, -(i as i64));
        }
        for l in 0..mx {
            for r in l+1..=mx {
                let tar_sum = ((r-1)*r)/2 - (l.max(1)-1)*l/2;
                assert_eq!(st.query(l..r), -(tar_sum as i64));
            }
        }
    }
}

mod lazy_seg_tree_tests {
    use crate::random::rand_u64;

    use super::LazySegTree;

    #[test]
    fn query_test() {
        let mx = 1000;
        let mut st = LazySegTree::from_iter(0..mx, |d1, d2| d1+d2, |l1, l2| l2, |d, l| l);
        for l in 0..mx {
            for r in l+1..=mx {
                let tar_sum = ((r-1)*r)/2 - (l.max(1)-1)*l/2;
                assert_eq!(st.query(l..r), tar_sum);
            }
        }
    }
    #[test]
    fn upd_test_simple() {
        let n = 10;
        let mut st = LazySegTree::from_iter((0..n).map(|i| (i as i64, 1)), |d1, d2| (d1.0+d2.0, d1.1+d2.1), |l1, l2| l2, |d, l| (l*d.1 as i64, d.1));
        st.update(0..n, -1);
        assert_eq!(st.query(0..=0).0, -1);
    }

    #[test]
    fn upd_test() {
        let n = 1000;
        let mut st = LazySegTree::from_iter((0..n).map(|i| (i as i64, 1)), |d1, d2| (d1.0+d2.0, d1.1+d2.1), |l1, l2| l2, |d, l| (l*d.1 as i64, d.1));
        for i in 0..n {
            st.update(i..(i*2+1).min(n), -(i as i64));
        }
        for l in 0..n {
            for r in l+1..=n {
                let tar_sum = ((r-1)*r)/2 - (l.max(1)-1)*l/2;
                assert_eq!(st.query(l..r).0, -(tar_sum as i64));
            }
        }
    }

    #[test]
    fn upd_test_max() {
        let n = 1000;
        let mut st = LazySegTree::from_iter(0..n as u64, |d1, d2| d1.max(d2), |l1: u64, l2| l2+l1, |d, l| d+l);

        for i in 0..n {
            for j in i+1..=n {
                assert_eq!(st.query(i..j), j as u64-1);
            }
        }
    }

    #[test]
    fn upd_test_max_randomized() {
        let n = 1000;
        let mut data = (0..n as u64).collect::<Vec<_>>();
        let mut st = LazySegTree::from_iter(data.clone(), |d1, d2| d1.max(d2), |l1: u64, l2| l1+l2, |d, l| l+d);


        let samples_cnt = 1000;
        for _ in 0..samples_cnt {
            let mut l = (rand_u64() as usize)%n;
            let mut r = (rand_u64() as usize)%n;
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            r += 1;

            if rand_u64()%2 == 0 {
                let val = rand_u64()%(u32::MAX as u64);

                st.update(l..r, val);
                for i in l..r {
                    data[i] += val;
                }
            }
            else {
                let res = st.query(l..r);
                let mut tar_res = 0;
                for i in l..r {
                    tar_res = tar_res.max(data[i]);
                }
                assert_eq!(tar_res, res);
            }
        }
    }
}
