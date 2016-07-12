use std::collections;

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
      node: &self,
      done: false,
      parents: Vec::new()
    }
  }
}

pub struct TreeIterator<'a> {
  parents: Vec<&'a TreeNode>,
  done: bool,
  node: &'a Tree
}

impl<'a> Iterator for TreeIterator<'a> {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<Self::Item> {
    match self.node.0 {
      None => None,

      Some(ref n) =>
        match n.left.0 {
          Some(ref t) => {
            self.node = &n.left;
            Some((t.key, t.value))
          }

          None => {
            match self.done {
              false => {
                self.done = true;
                Some((n.key, n.value))
              }

              true => {
                None
              }
            }
          }
        }
    }
  }
}

#[test]
fn iterator_empty() {
  let t = Tree::new();
  let mut iter = t.iter();
  assert_eq!(iter.next(), None);
}

#[test]
fn iterator_single() {
  let mut t = Tree::new();
  t.set(1,2);
  let mut iter = t.iter();
  assert_eq!(iter.next(), Some((1,2)));
  assert_eq!(iter.next(), None);
}

#[test]
fn iterator_left() {
  let mut t = Tree::new();
  t.set(2,3);
  t.set(1,2);
  let mut iter = t.iter();
  assert_eq!(iter.next(), Some((1,2)));
  assert_eq!(iter.next(), Some((2,3)));
  assert_eq!(iter.next(), None);
}

#[test]
fn iterator_right() {
  let mut t = Tree::new();
  t.set(1,2);
  t.set(2,3);
  let mut iter = t.iter();
  assert_eq!(iter.next(), Some((1,2)));
  assert_eq!(iter.next(), Some((2,3)));
  assert_eq!(iter.next(), None);
}

#[test]
fn iterator_left_right() {
  let mut t = Tree::new();
  t.set(1,2);
  t.set(2,3);
  t.set(3,4);
  let mut iter = t.iter();
  assert_eq!(iter.next(), Some((1,2)));
  assert_eq!(iter.next(), Some((2,3)));
  assert_eq!(iter.next(), Some((3,4)));
  assert_eq!(iter.next(), None);
}

#[test]
fn iterator_left_left() {
  let mut t = Tree::new();
  t.set(3,4);
  t.set(2,3);
  t.set(1,2);
  let mut iter = t.iter();
  assert_eq!(iter.next(), Some((1,2)));
  assert_eq!(iter.next(), Some((2,3)));
  assert_eq!(iter.next(), Some((3,4)));
  assert_eq!(iter.next(), None);
}

#[test]
fn iterator_left_left_right_right() {
  let mut t = Tree::new();
  t.set(3,4);
  t.set(2,3);
  t.set(1,2);
  t.set(4,5);
  t.set(5,6);
  let mut iter = t.iter();
  assert_eq!(iter.next(), Some((1,2)));
  assert_eq!(iter.next(), Some((2,3)));
  assert_eq!(iter.next(), Some((3,4)));
  assert_eq!(iter.next(), Some((4,5)));
  assert_eq!(iter.next(), Some((5,6)));
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
