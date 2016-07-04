#[derive(Debug)]
enum Tree {
  Empty,
  Node(Box<TreeNode>)
}

#[derive(Debug)]
struct TreeNode {
  key: u32,
  value: u32,
  left: Tree,
  right: Tree
}

impl Tree {
  fn new() -> Tree {
    Tree::Empty
  }

  fn get(self: &Tree, k: u32) -> Option<u32> {
    match self {
      &Tree::Empty => None,
      &Tree::Node(ref nt) => {
        let t = nt.as_ref();
        if (k == t.key) {
          Some(t.value)
        } else {
          let n = {
            if (k < t.key) {
              &t.left
            } else {
              &t.right
            }
          };
          n.get(k)
        }
      }
    }
  }

//  fn set_helper(t: &TreeNode) -> Option<Box<TreeNode>> {
//  }

  fn set(self: &mut Tree, k: u32, v: u32) {
    match self {
      &mut Tree::Empty => {
        let mut n = TreeNode {key: k, value: v, left: Tree::Empty, right: Tree::Empty};
        self = Tree::Node(Box::new(n));
      },
      &mut Tree::Node(ref nt) => {
        let t = nt.as_ref();
        if (k == t.key) {
          t.value = v
        } else {
          t.left.set(k, v)
        }
      }
    }
  }
}

#[test]
fn new_empty() {
  let mut t = Tree::new();
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
  let v = t.get(1);
  println!("{:?}", v);
  assert_eq!(v.unwrap(), 41);
}

#[test]
fn set_get_1_1() {
  let mut t = Tree::new();
  t.set(1,42);
  t.set(1,41);
  assert_eq!(t.get(1).unwrap(), 41);
}

#[test]
fn set_get_1_2() {
  let mut t = Tree::new();
  t.set(1,41);
  t.set(2,42);
  assert_eq!(t.get(1).unwrap(), 41);
  assert_eq!(t.get(2).unwrap(), 42);
}

#[test]
fn set_get_1_3_2() {
  let mut t = Tree::new();
  t.set(1,41);
  t.set(3,43);
  t.set(2,42);
  assert_eq!(t.get(1).unwrap(), 41);
  assert_eq!(t.get(2).unwrap(), 42);
  assert_eq!(t.get(3).unwrap(), 43);
}
