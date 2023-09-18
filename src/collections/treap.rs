use std::mem;

use super::super::misc::random::rand_u64;

pub enum NodeCont<T: PartialOrd> {
    Some(Box<Node<T>>),
    Empty
}

impl<T: PartialOrd> Default for NodeCont<T> {
    fn default() -> Self {
        NodeCont::Empty
    }
}

// if key == other_key then it will be in the left subtree
pub struct Treap<T: PartialOrd> {
    root: NodeCont<T>
}

impl<T: Clone+PartialOrd> Treap<T> {
    pub fn new() -> Self {
        Self {
            root: NodeCont::Empty
        }
    }
    pub fn from_root(root: NodeCont<T>) -> Self {
        Self {
            root
        }
    }
    /// Returns two treaps such that all the keys in the first treap are smaller than or equal to x and in the second treap higher than x
    pub fn split(self, x: T) -> (Treap<T>, Treap<T>) {
        let (left_root, right_root) = self.root.split(x);
        (Treap::from_root(left_root), Treap::from_root(right_root))
    }
    pub fn insert(&mut self, key: T) {
        let root = mem::take(&mut self.root);
        self.root = root.insert(Node::from_key(key));
    }
    /// All the keys in the left treap must be lower than all the keys in the right treap
    pub fn merge(self, other: Treap<T>) -> Treap<T> {
        let new_root = self.root.merge(other.root);
        Self {root: new_root}
    }
    pub fn erase(&mut self, key: &T) {
        let root = mem::take(&mut self.root);
        self.root = root.erase(key);
    }
    pub fn contains(&self, key: &T) -> bool {
        self.root.contains(key)
    }
}

pub struct Node<T: PartialOrd> {
    key: T,
    left: NodeCont<T>,
    right: NodeCont<T>,
    priority: u64,
}

impl<T: PartialOrd> Node<T> {
    pub fn from_key(key: T) -> Self {
        Self {
            key, 
            left: NodeCont::Empty,
            right: NodeCont::Empty,
            priority: rand_u64()
        }
    }
}

impl<T: PartialOrd + Clone> NodeCont<T> {
    fn split(self, x: T) -> (NodeCont<T>, NodeCont<T>) {
        let Self::Some(mut root) = self else {
            return (Self::Empty, Self::Empty);
        };

        let (left_res, right_res);
        if root.key > x {
            (left_res, root.left) = root.left.split(x);
            right_res = NodeCont::Some(root);
        }
        else {
            (root.right, right_res) = root.right.split(x);
            left_res = NodeCont::Some(root);
        }

        (left_res, right_res)
    }
    fn insert(self, mut single_node: Node<T>) -> NodeCont<T> {
        let NodeCont::Some(mut root) = self else {
            return NodeCont::Some(Box::new(single_node));
        };

        if single_node.priority >= root.priority {
            (single_node.left, single_node.right) = NodeCont::Some(root).split(single_node.key.clone());
            NodeCont::Some(Box::new(single_node))
        }
        else {
            if single_node.key <= root.key {
                root.left = root.left.insert(single_node);
            }
            else {
                root.right = root.right.insert(single_node);
            }
            NodeCont::Some(root)
        }
    }
    fn contains(&self, key: &T) -> bool {
        let NodeCont::Some(root) = self else {
            return false;
        };

        match key.partial_cmp(&root.key).unwrap() {
            std::cmp::Ordering::Less => {
                root.left.contains(key)
            },
            std::cmp::Ordering::Greater => {
                root.right.contains(key)
            },
            std::cmp::Ordering::Equal => {
                true
            },
        }
    }
    fn erase(self, key: &T) -> NodeCont<T> {
        let NodeCont::Some(mut root) = self else {
            return NodeCont::Empty;
        };

        match key.partial_cmp(&root.key).unwrap() {
            std::cmp::Ordering::Less => {
                root.left = root.left.erase(key);
                NodeCont::Some(root)
            },
            std::cmp::Ordering::Greater => {
                root.right = root.right.erase(key);
                NodeCont::Some(root)
            },
            std::cmp::Ordering::Equal => {
                let left_subtree = mem::take(&mut root.left);
                let right_subtree = mem::take(&mut root.right);
                left_subtree.merge(right_subtree)
            },
        }
    }
    fn merge(self, other: NodeCont<T>) -> NodeCont<T> {
        let NodeCont::Some(mut left_root) = self else {
            return other;
        };
        let NodeCont::Some(mut right_root) = other else {
            return NodeCont::Some(left_root);
        };
        if left_root.priority >= right_root.priority {
            let (left_child, right_child) = (mem::take(&mut left_root.left), mem::take(&mut left_root.right));
            left_root.left = left_child.merge(right_child);
            left_root.right = NodeCont::Some(right_root);
            NodeCont::Some(left_root)
        }
        else {
            let (left_child, right_child) = (mem::take(&mut right_root.left), mem::take(&mut right_root.right));
            right_root.right = left_child.merge(right_child);
            right_root.left = NodeCont::Some(left_root);
            NodeCont::Some(right_root)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_treap() {
        use super::Treap;
        let mut treap = Treap::new();
        treap.insert(1);
        treap.insert(2);
        treap.insert(3);
        assert!(treap.contains(&1));
        assert!(treap.contains(&2));
        assert!(treap.contains(&3));
        assert!(!treap.contains(&4));
        treap.erase(&2);
        assert!(!treap.contains(&2));
        treap.insert(2);
        assert!(treap.contains(&2));
        treap.erase(&1);
        assert!(!treap.contains(&1));
        treap.erase(&2);
        assert!(!treap.contains(&2));
        treap.erase(&3);
        assert!(!treap.contains(&3));
        treap.insert(2);
        treap.insert(1);
        treap.insert(3);
        assert!(treap.contains(&1));
        assert!(treap.contains(&2));
        assert!(treap.contains(&3));
        treap.insert(4);
        treap.insert(5);
        treap.insert(6);
        assert!(treap.contains(&4));
        assert!(treap.contains(&5));
        assert!(treap.contains(&6));
        assert!(!treap.contains(&7));
    }
    
}