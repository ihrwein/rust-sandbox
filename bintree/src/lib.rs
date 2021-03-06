#[derive(Debug)]
pub struct Tree<K: Ord, V>(TreeKind<K, V>);

type TreeKind<K, V> = Option<Box<TreeNode<K, V>>>;

#[derive(Debug)]
pub struct TreeNode<K: Ord, V> {
    key: K,
    value: V,
    left: Tree<K, V>,
    right: Tree<K, V>,
}


impl<K: Clone + Ord, V: Clone> Clone for Tree<K, V> {
    fn clone(&self) -> Tree<K, V> {
        Tree(self.0.clone())
    }
}

impl<K: Clone + Ord, V: Clone> Clone for TreeNode<K, V> {
    fn clone(&self) -> TreeNode<K, V> {
        TreeNode {
            key: self.key.clone(),
            value: self.value.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Tree<K, V> {
        Tree(None)
    }

    pub fn get(&self, k: K) -> Option<&V> {
        self.0.as_ref().map_or(None, |n| {
            if k == n.key {
                Some(&n.value)
            } else if k < n.key {
                n.left.get(k)
            } else {
                n.right.get(k)
            }
        })
    }

    pub fn set(&mut self, k: K, v: V) {
        match self.0 {
            None => {
                let n = TreeNode {
                    key: k,
                    value: v,
                    left: Tree::new(),
                    right: Tree::new(),
                };
                self.0 = Some(Box::new(n));
            }

            Some(ref mut n) => {
                if k == n.key {
                    n.value = v
                } else if k < n.key {
                    n.left.set(k, v)
                } else {
                    n.right.set(k, v)
                }
            }
        }
    }

    pub fn inorder<F: FnMut(&K, &V)>(&self, callback: &mut F) {
        if let Some(ref n) = self.0 {
            n.left.inorder(callback);
            callback(&n.key, &n.value);
            n.right.inorder(callback);
        }
    }

    pub fn inorder_eat<F: FnMut(K, V)>(self, callback: &mut F) {
        if let Some(n) = self.0 {
            // workaround for https://github.com/rust-lang/rust/issues/16223
            let _n = *n;
            let TreeNode{key, value, left, right} = _n;
            left.inorder_eat(callback);
            callback(key, value);
            right.inorder_eat(callback);
        }
    }

    pub fn inorder_iter<'a>(&'a self) -> InorderIter<'a, K, V> {
        InorderIter {stack: Vec::new(), node: self.0.as_ref()}
    }

    pub fn inorder_eating_iter(self) -> EatingInorderIter<K, V> {
        EatingInorderIter { stack: Vec::new(), node: self.0}
    }
}

pub struct InorderIter<'a, K: Ord + 'a, V: 'a> {
    stack: Vec<&'a Box<TreeNode<K, V>>>,
    node: Option<&'a Box<TreeNode<K, V>>>
}

impl<'a, K: Ord, V> Iterator for InorderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(n) = self.node {
                self.stack.push(n);
                self.node = n.left.0.as_ref();
            } else if let Some(n) = self.stack.pop() {
                self.node = n.right.0.as_ref();
                return Some((&n.key, &n.value));
            } else {
                return None;
            }
        }
    }
}

pub struct EatingInorderIter<K: Ord, V> {
    stack: Vec<Box<TreeNode<K, V>>>,
    node: Option<Box<TreeNode<K, V>>>
}

impl<K: Ord, V> Iterator for EatingInorderIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut n) = self.node.take() {
                self.node = n.left.0.take();
                self.stack.push(n);
            } else if let Some(mut n) = self.stack.pop() {
                self.node = n.right.0.take();
                let _n = *n;
                let TreeNode{key, value, left, right} = _n;
                let (_,_) = (left, right);
                return Some((key, value));
            } else {
                return None;
            }
        }
    }
}

pub fn iterative_inorder<K: Ord, V, F: FnMut((&K, &V))>(mut node: Option<&Box<TreeNode<K, V>>>, mut cb: F) {
    let mut stack = Vec::new();

    loop {
        if let Some(n) = node {
            stack.push(n);
            node = n.left.0.as_ref();
        } else if let Some(n) = stack.pop() {
            cb((&n.key, &n.value));
            node = n.right.0.as_ref();
        } else {
            return;
        }
    }
}

/*
iterativeInorder(node)
  s ← empty stack
  while (not s.isEmpty() or node ≠ null)
    if (node ≠ null)
      s.push(node)
      node ← node.left
    else
      node ← s.pop()
      visit(node)
      node ← node.right
*/

pub fn gen_bin_tree(depth: u32) -> Tree<i32, i32> {
    let node = bin_tree_factory::<i32, i32>(2_i32.pow(depth), depth);
    Tree(Some(Box::new(node)))
}

fn bin_tree_factory<K: Ord, V>(root: i32, depth: u32) -> TreeNode<i32, i32> {
    if depth == 0 {
        TreeNode {
            key: root,
            value: root,
            left: Tree(None),
            right: Tree(None)
        }
    } else {
        let left_child = bin_tree_factory::<i32, i32>(root - 2_i32.pow(depth - 1), depth - 1);
        let right_child = bin_tree_factory::<i32, i32>(root + 2_i32.pow(depth - 1), depth - 1);
        TreeNode {
            key: root,
            value: root,
            left: Tree(Some(Box::new(left_child))),
            right: Tree(Some(Box::new(right_child))),
        }
    }
}



#[test]
fn new_empty() {
    let t: Tree<_, i32> = Tree::new();
    assert_eq!(t.get(0), None);
}

#[test]
fn set_get_missing() {
    let mut t = Tree::new();
    t.set(1, 2);
    let v = t.get(3);
    assert_eq!(v, None);
}

#[test]
fn set_get_1() {
    let mut t = Tree::new();
    t.set(1, 41);
    assert_eq!(t.get(1).unwrap(), &41);
}

#[test]
fn set_get_1_1() {
    let mut t = Tree::new();
    t.set(1, 42);
    t.set(1, 41);
    assert_eq!(t.get(1).unwrap(), &41);
}

#[test]
fn set_get_1_2() {
    let mut t = Tree::new();
    t.set(1, 41);
    t.set(2, 42);
    assert_eq!(t.get(1).unwrap(), &41);
    assert_eq!(t.get(2).unwrap(), &42);
}

#[test]
fn set_get_1_3_2() {
    let mut t = Tree::new();
    t.set(1, 41);
    t.set(3, 43);
    t.set(2, 42);
    println!("{:?}", t);
    assert_eq!(t.get(1).unwrap(), &41);
    assert_eq!(t.get(2).unwrap(), &42);
    assert_eq!(t.get(3).unwrap(), &43);
}

#[test]
fn test_recursive_inorder_traversal() {
    let mut t = Tree::new();
    t.set(1, 41);
    t.set(3, 43);
    t.set(2, 42);

    let expected_order = [1, 2, 3];
    let mut traversed_keys = Vec::new();

    t.inorder(&mut |k, _| {
        traversed_keys.push(*k);
    });

    assert_eq!(&expected_order[..], &traversed_keys[..]);
}

#[test]
fn test_eating_recursive_inorder_traversal() {
    let mut t = Tree::new();
    t.set(1, "41".to_string());
    t.set(3, "43".to_string());
    t.set(2, "42".to_string());

    let expected_values = ["41".to_string(), "42".to_string(), "43".to_string()];
    let mut traversed_values = Vec::new();

    t.inorder_eat(&mut |_, v| {
        traversed_values.push(v);
    });

    // t.inorder_eat(&mut |_, _| {
    //     println!("This shouldn't compile as we eat the tree with the previous traversal");
    // });

    assert_eq!(&expected_values[..], &traversed_values[..]);
}

#[test]
fn test_iterative_inorder() {
    let mut t = Tree::new();
    t.set(1, 41);
    t.set(3, 43);
    t.set(2, 42);

    let expected_order = [1, 2, 3];
    let mut traversed_keys = Vec::<i32>::new();

    iterative_inorder(t.0.as_ref(), |(k, _)| {
        traversed_keys.push(*k);
    });

    assert_eq!(&expected_order[..], &traversed_keys[..]);
}

#[test]
fn test_inorder_iterator() {
    let mut t = Tree::new();
    t.set(1, 41);
    t.set(3, 43);
    t.set(2, 42);

    let expected_order = [1, 2, 3];
    let traversed_keys = t.inorder_iter().map(|(k, _)| *k).collect::<Vec<i32>>();

    assert_eq!(&expected_order[..], &traversed_keys[..]);
}

#[test]
fn test_eating_inorder_traversal() {
    let mut t = Tree::new();
    t.set(1, "41".to_string());
    t.set(3, "43".to_string());
    t.set(2, "42".to_string());

    let expected_values = ["41".to_string(), "42".to_string(), "43".to_string()];
    let traversed_values = t.inorder_eating_iter().map(|(_, v)| v).collect::<Vec<String>>();

    // t.inorder_eat(&mut |_, _| {
    //     println!("This shouldn't compile as we eat the tree with the previous traversal");
    // });

    assert_eq!(&expected_values[..], &traversed_values[..]);
}

#[test]
fn test_bin_tree_factory() {
    let tree = gen_bin_tree(9);
    let kv_pairs = tree.inorder_eating_iter().collect::<Vec<(i32, i32)>>();
    let expected = (1..1024).map(|v| (v, v)).collect::<Vec<(i32, i32)>>();
    assert_eq!(expected, kv_pairs);
}
