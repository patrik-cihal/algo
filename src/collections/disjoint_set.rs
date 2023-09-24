pub struct DisjointSet {
     data: Vec<usize>,
     size: Vec<usize>
}

impl DisjointSet {
     pub fn new(n: usize) -> Self {
          Self {
               data: (0..n).collect(),
               size: vec![1; n]
          }
     }
     pub fn find(&mut self, mut x: usize) -> usize {
          if self.data[x] != x {
               x = self.find(self.data[x]);
               self.data[x] = x;
          }
          x
     }
     pub fn join(&mut self, mut x: usize, mut y: usize) -> bool {
          x = self.find(x);
          y = self.find(y);

          if x==y {
               return false;
          }

          if self.size[x] < self.size[y] {
               std::mem::swap(&mut x, &mut y);
          }

          self.size[x] += self.size[y];
          self.data[y] = x;

          true
     }
     pub fn size(&mut self, x: usize) -> usize {
          let x = self.find(x);
          self.size[x]
     }
     pub fn len(&self) -> usize {
          self.data.len()
     }
}