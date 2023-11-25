use std::ops::RangeBounds;

pub struct SparseTable<T: Clone, F: Fn(T, T) -> T> {
    data: Vec<Vec<T>>,
    f: F 
}

impl<T: Clone, F: Fn(T, T) -> T> SparseTable<T, F> {
    pub fn new_from_iter(iter: impl IntoIterator<Item=T>,f: F) -> Self {
        let data_init = iter.into_iter().collect::<Vec<_>>();
        assert!(data_init.len() != 0);
        let lg = (data_init.len().ilog2()+1) as usize;
        let mut data = data_init.into_iter().map(|val| vec![val; lg]).collect::<Vec<_>>();
        for i in 1..lg {
            let offs = 1<<(i-1);
            for j in 0..data.len()-offs {
                data[j][i] = (f)(data[j][i-1].clone(), data[j+offs][i-1].clone());
            }
        }
        Self {
            data,
            f
        }
    }
    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        let l = match range.start_bound() {
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(s) => *s+1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(e) => *e,
            std::ops::Bound::Excluded(e) => *e-1,
            std::ops::Bound::Unbounded => self.data.len()-1,
        };
        if l==r {
            return self.data[l][0].clone();
        }

        let dist = r-l;

        let i = dist.ilog2() as usize;

        (self.f)(self.data[l][i].clone(), self.data[r-(1<<i)][i].clone())
    }
}


#[cfg(test)]
mod tests {
    use super::SparseTable;

    #[test]
    fn query() {
        let sparse_table = SparseTable::new_from_iter(vec![0, 2, 3, 5], |a, b| a.min(b));

        assert!(sparse_table.query(0..=3) == 0);
        assert!(sparse_table.query(..) == 0);
        assert!(sparse_table.query(3..) == 5);
        assert!(sparse_table.query(2..) == 3);
        assert!(sparse_table.query(1..=1) == 2);
    }
}