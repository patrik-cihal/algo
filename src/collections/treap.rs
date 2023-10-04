use std::mem;

use super::super::misc::random::rand_u64;

pub enum TreapNodeCont<T: PartialOrd> {
    Some(Box<TreapNode<T>>),
    Empty,
}

impl<T: PartialOrd> Default for TreapNodeCont<T> {
    fn default() -> Self {
        TreapNodeCont::Empty
    }
}

// if key == other_key then it will be in the left subtree
pub struct Treap<T: PartialOrd> {
    root: TreapNodeCont<T>,
}

impl<T: Clone + PartialOrd> Treap<T> {
    pub fn new() -> Self {
        Self {
            root: TreapNodeCont::Empty,
        }
    }
    pub fn from_root(root: TreapNodeCont<T>) -> Self {
        Self { root }
    }
    /// Returns two treaps such that all the keys in the first treap are smaller than or equal to x and in the second treap higher than x
    pub fn split(self, x: T) -> (Treap<T>, Treap<T>) {
        let (left_root, right_root) = self.root.split(x);
        (Treap::from_root(left_root), Treap::from_root(right_root))
    }
    pub fn insert(&mut self, key: T) {
        let root = mem::take(&mut self.root);
        self.root = root.insert(TreapNode::from_key(key));
    }
    /// All the keys in the left treap must be lower than all the keys in the right treap
    pub fn merge(self, other: Treap<T>) -> Treap<T> {
        let new_root = self.root.merge(other.root);
        Self { root: new_root }
    }
    pub fn erase(&mut self, key: &T) {
        let root = mem::take(&mut self.root);
        self.root = root.erase(key);
    }
    pub fn contains(&self, key: &T) -> bool {
        self.root.contains(key)
    }
    pub fn len(&self) -> usize {
        self.root.len()
    }
}

pub struct TreapNode<T: PartialOrd> {
    key: T,
    left: TreapNodeCont<T>,
    right: TreapNodeCont<T>,
    priority: u64,
    len: usize,
}

impl<T: PartialOrd + Clone> TreapNode<T> {
    pub fn from_key(key: T) -> Self {
        Self {
            key,
            left: TreapNodeCont::Empty,
            right: TreapNodeCont::Empty,
            priority: rand_u64(),
            len: 1,
        }
    }
    fn recompute(&mut self) {
        self.len = self.left.len() + self.right.len() + 1;
    }
}

impl<T: PartialOrd + Clone> TreapNodeCont<T> {
    fn len(&self) -> usize {
        if let TreapNodeCont::Some(root) = self {
            root.len
        } else {
            0
        }
    }
    fn split(self, x: T) -> (TreapNodeCont<T>, TreapNodeCont<T>) {
        let Self::Some(mut root) = self else {
            return (Self::Empty, Self::Empty);
        };

        let (left_res, right_res);
        if root.key > x {
            (left_res, root.left) = root.left.split(x);
            root.len = root.left.len() + root.right.len();
            root.recompute();
            right_res = TreapNodeCont::Some(root);
        } else {
            (root.right, right_res) = root.right.split(x);
            root.recompute();
            left_res = TreapNodeCont::Some(root);
        }

        (left_res, right_res)
    }
    fn insert(self, mut single_node: TreapNode<T>) -> TreapNodeCont<T> {
        let TreapNodeCont::Some(mut root) = self else {
            return TreapNodeCont::Some(Box::new(single_node));
        };

        if single_node.priority >= root.priority {
            (single_node.left, single_node.right) =
                TreapNodeCont::Some(root).split(single_node.key.clone());
            single_node.recompute();
            TreapNodeCont::Some(Box::new(single_node))
        } else {
            if single_node.key <= root.key {
                root.left = root.left.insert(single_node);
            } else {
                root.right = root.right.insert(single_node);
            }
            root.recompute();
            TreapNodeCont::Some(root)
        }
    }
    fn contains(&self, key: &T) -> bool {
        let TreapNodeCont::Some(root) = self else {
            return false;
        };

        match key.partial_cmp(&root.key).unwrap() {
            std::cmp::Ordering::Less => root.left.contains(key),
            std::cmp::Ordering::Greater => root.right.contains(key),
            std::cmp::Ordering::Equal => true,
        }
    }
    fn erase(self, key: &T) -> TreapNodeCont<T> {
        let TreapNodeCont::Some(mut root) = self else {
            return TreapNodeCont::Empty;
        };

        match key.partial_cmp(&root.key).unwrap() {
            std::cmp::Ordering::Less => {
                root.left = root.left.erase(key);
                root.recompute();
                TreapNodeCont::Some(root)
            }
            std::cmp::Ordering::Greater => {
                root.right = root.right.erase(key);
                root.recompute();
                TreapNodeCont::Some(root)
            }
            std::cmp::Ordering::Equal => {
                let left_subtree = mem::take(&mut root.left);
                let right_subtree = mem::take(&mut root.right);
                left_subtree.merge(right_subtree)
            }
        }
    }
    fn merge(self, other: TreapNodeCont<T>) -> TreapNodeCont<T> {
        let TreapNodeCont::Some(mut left_root) = self else {
            return other;
        };
        let TreapNodeCont::Some(mut right_root) = other else {
            return TreapNodeCont::Some(left_root);
        };
        if left_root.priority >= right_root.priority {
            let (left_child, right_child) = (
                mem::take(&mut left_root.left),
                mem::take(&mut left_root.right),
            );
            left_root.left = left_child.merge(right_child);
            left_root.right = TreapNodeCont::Some(right_root);
            left_root.recompute();
            TreapNodeCont::Some(left_root)
        } else {
            let (left_child, right_child) = (
                mem::take(&mut right_root.left),
                mem::take(&mut right_root.right),
            );
            right_root.right = left_child.merge(right_child);
            right_root.left = TreapNodeCont::Some(left_root);
            right_root.recompute();
            TreapNodeCont::Some(right_root)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut treap: Treap<i32> = Treap::new();
        treap.insert(10);
        assert!(treap.contains(&10));
        assert!(!treap.contains(&5));

        treap.insert(5);
        assert!(treap.contains(&5));
        assert_eq!(treap.len(), 2);
    }

    #[test]
    fn test_erase() {
        let mut treap: Treap<i32> = Treap::new();
        treap.insert(10);
        treap.insert(5);
        treap.erase(&5);

        assert!(!treap.contains(&5));
        assert_eq!(treap.len(), 1);
    }

    #[test]
    fn test_split() {
        let mut treap: Treap<i32> = Treap::new();
        treap.insert(10);
        treap.insert(5);
        treap.insert(15);

        let (left, right) = treap.split(10);

        assert!(left.contains(&10));
        assert!(left.contains(&5));
        assert!(!right.contains(&10));
        assert!(right.contains(&15));
    }

    #[test]
    fn test_merge() {
        let mut left: Treap<i32> = Treap::new();
        let mut right: Treap<i32> = Treap::new();

        left.insert(5);
        right.insert(15);

        let merged = left.merge(right);

        assert!(merged.contains(&5));
        assert!(merged.contains(&15));
        assert_eq!(merged.len(), 2);
    }
}
