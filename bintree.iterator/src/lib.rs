#[derive(Debug)]
pub struct Tree (TreeKind);

type TreeKind = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
  key: i32,
  value: i32,
  left: Tree,
  right: Tree
}


impl Clone for Tree {
  fn clone(&self) -> Tree {
    Tree(self.0.clone())
  }
}

impl Clone for TreeNode {
  fn clone(&self) -> TreeNode {
    TreeNode {
      key: self.key.clone(),
      value: self.value.clone(),
      left: self.left.clone(),
      right: self.right.clone()
    }
  }
}

impl Tree {
  pub fn new() -> Tree {
    Tree(None)
  }

  pub fn get(&self, k: i32) -> Option<&i32> {
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

  pub fn set(&mut self, k: i32, v: i32) {
    match self.0 {
      None => {
        let n = TreeNode {
          key: k,
          value: v,
          left: Tree::new(),
          right: Tree::new()
        };
        self.0 = Some(Box::new(n));
      },

      Some(ref mut n) =>
        if k == n.key {
          n.value = v
        } else if k < n.key {
          n.left.set(k, v)
        } else {
          n.right.set(k, v)
        }
    }
  }

  pub fn iter(&self) -> TreeIterator {
    TreeIterator {
      node: None
    }
  }
}

pub struct TreeIterator {
  node: Option<TreeNode>
}

impl Iterator for TreeIterator {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}

#[test]
fn iterator_empty() {
  let t = Tree::new();
  let mut iter = t.iter();
  assert_eq!(iter.next(), None);
}

#[test]
fn new_empty() {
  let t: Tree = Tree::new();
  assert_eq!(t.get(0), None);
}

#[test]
fn set_get_missing() {
  let mut t = Tree::new();
  t.set(1,2);
  let v = t.get(3);
  assert_eq!(v, None);
}

#[test]
fn set_get_1() {
  let mut t = Tree::new();
  t.set(1,41);
  assert_eq!(t.get(1).unwrap(), &41);
}

#[test]
fn set_get_1_1() {
  let mut t = Tree::new();
  t.set(1,42);
  t.set(1,41);
  assert_eq!(t.get(1).unwrap(), &41);
}

#[test]
fn set_get_1_2() {
  let mut t = Tree::new();
  t.set(1,41);
  t.set(2,42);
  assert_eq!(t.get(1).unwrap(), &41);
  assert_eq!(t.get(2).unwrap(), &42);
}

#[test]
fn set_get_1_3_2() {
  let mut t = Tree::new();
  t.set(1,41);
  t.set(3,43);
  t.set(2,42);
  println!("{:?}", t);
  assert_eq!(t.get(1).unwrap(), &41);
  assert_eq!(t.get(2).unwrap(), &42);
  assert_eq!(t.get(3).unwrap(), &43);
}
